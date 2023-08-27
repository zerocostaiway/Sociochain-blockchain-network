// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! This module defines `HostState` and `HostContext` structs which provide logic and state
//! required for execution of host.

use crate::{instance_wrapper::MemoryWrapper, runtime::StoreData, util};
use sc_allocator::{AllocationStats, FreeingBumpHeapAllocator};
use sp_io::{RiscvExecOutcome, RiscvState};
use sp_wasm_interface::{Pointer, WordSize};
use std::{mem, any::Any};
use wasmtime::{AsContext, Caller, TypedFunc};

/// The state required to construct a HostContext context. The context only lasts for one host
/// call, whereas the state is maintained for the duration of a Wasm runtime call, which may make
/// many different host calls that must share state.
pub struct HostState {
	/// The allocator instance to keep track of allocated memory.
	///
	/// This is stored as an `Option` as we need to temporarly set this to `None` when we are
	/// allocating/deallocating memory. The problem being that we can only mutable access `caller`
	/// once.
	allocator: Option<FreeingBumpHeapAllocator>,
	panic_message: Option<String>,
	riscv_data: Option<*mut RiscvData>,
}

unsafe impl Send for HostState {}

impl HostState {
	/// Constructs a new `HostState`.
	pub fn new(allocator: FreeingBumpHeapAllocator) -> Self {
		HostState {
			allocator: Some(allocator),
			panic_message: None,
			riscv_data: None,
		}
	}

	/// Takes the error message out of the host state, leaving a `None` in its place.
	pub fn take_panic_message(&mut self) -> Option<String> {
		self.panic_message.take()
	}

	pub(crate) fn allocation_stats(&self) -> AllocationStats {
		self.allocator.as_ref()
			.expect("Allocator is always set and only unavailable when doing an allocation/deallocation; qed")
			.stats()
	}
}

/// A `HostContext` implements `FunctionContext` for making host calls from a Wasmtime
/// runtime. The `HostContext` exists only for the lifetime of the call and borrows state from
/// a longer-living `HostState`.
pub(crate) struct HostContext<'a> {
	pub(crate) caller: Caller<'a, StoreData>,
}

impl<'a> HostContext<'a> {
	fn host_state_mut(&mut self) -> &mut HostState {
		self.caller
			.data_mut()
			.host_state_mut()
			.expect("host state is not empty when calling a function in wasm; qed")
	}

	fn riscv_data_mut<'b, 'c>(&'b mut self) -> Option<&'c mut RiscvData> {
		self.host_state_mut().riscv_data.map(|i| unsafe { &mut *i })
	}
}

impl<'a> sp_wasm_interface::FunctionContext for HostContext<'a> {
	fn read_memory_into(
		&self,
		address: Pointer<u8>,
		dest: &mut [u8],
	) -> sp_wasm_interface::Result<()> {
		util::read_memory_into(&self.caller, address, dest).map_err(|e| e.to_string())
	}

	fn write_memory(&mut self, address: Pointer<u8>, data: &[u8]) -> sp_wasm_interface::Result<()> {
		util::write_memory_from(&mut self.caller, address, data).map_err(|e| e.to_string())
	}

	fn allocate_memory(&mut self, size: WordSize) -> sp_wasm_interface::Result<Pointer<u8>> {
		let memory = self.caller.data().memory();
		let mut allocator = self
			.host_state_mut()
			.allocator
			.take()
			.expect("allocator is not empty when calling a function in wasm; qed");

		// We can not return on error early, as we need to store back allocator.
		let res = allocator
			.allocate(&mut MemoryWrapper(&memory, &mut self.caller), size)
			.map_err(|e| e.to_string());

		self.host_state_mut().allocator = Some(allocator);

		res
	}

	fn deallocate_memory(&mut self, ptr: Pointer<u8>) -> sp_wasm_interface::Result<()> {
		let memory = self.caller.data().memory();
		let mut allocator = self
			.host_state_mut()
			.allocator
			.take()
			.expect("allocator is not empty when calling a function in wasm; qed");

		// We can not return on error early, as we need to store back allocator.
		let res = allocator
			.deallocate(&mut MemoryWrapper(&memory, &mut self.caller), ptr)
			.map_err(|e| e.to_string());

		self.host_state_mut().allocator = Some(allocator);

		res
	}

	fn register_panic_error_message(&mut self, message: &str) {
		self.host_state_mut().panic_message = Some(message.to_owned());
	}

	fn riscv(&mut self) -> &mut dyn sp_wasm_interface::Riscv {
		self
	}
}

impl<'a> sp_wasm_interface::Riscv for HostContext<'a> {
	fn execute(
		&mut self,
		program: &[u8],
		function: &str,
		syscall_handler: u32,
		state_ptr: u32,
	) -> sp_wasm_interface::Result<u8> {
		// Extract a syscall handler from the instance's table by the specified index.
		let syscall_handler = {
			let table = self
				.caller
				.data()
				.table
				.ok_or("Runtime doesn't have a table; sandbox is unavailable")?;
			let table_item = table.get(&mut self.caller, syscall_handler);

			table_item
				.ok_or("dispatch_thunk_id is out of bounds")?
				.funcref()
				.ok_or("dispatch_thunk_idx should be a funcref")?
				.ok_or("dispatch_thunk_idx should point to actual func")?
				.typed(&mut self.caller)
				.map_err(|_| "dispatch_thunk_idx has the wrong type")?
		};

		let program = match polkavm::ProgramBlob::parse(program) {
			Ok(program) => program,
			Err(err) => {
				log::error!("Failed to parse polkavm program: {}", err);
				return Ok(RiscvExecOutcome::InvalidImage as u8)
			}
		};
		let config = polkavm::Config::new();
		let engine = polkavm::Engine::new(&config).unwrap();
		let module = polkavm::Module::from_blob(&engine, &program).unwrap();

		let mut linker = polkavm::Linker::new(&engine);
		linker.func_fallback(on_ecall);
		let instance = match linker.instantiate_pre(&module) {
			Ok(instance) => instance,
			Err(err) => {
				log::error!("Failed to link polkavm program: {}", err);
				return Ok(RiscvExecOutcome::InvalidImage as u8)
			}
		};
		let instance = instance.instantiate().unwrap();

		let func = match instance.get_typed_func::<(), ()>(function) {
			Ok(func) => func,
			Err(err) => {
				log::error!("Failed to find exported function: {}", err);
				return Ok(RiscvExecOutcome::InvalidImage as u8)
			}
		};

		let mut context = RiscvContext {
			host: self,
			data: RiscvData {
				syscall_handler,
				state_ptr,
				caller: None,
			},
		};

		let outcome = match func.call(&mut context, ()) {
			Ok(_) => RiscvExecOutcome::Ok,
			Err(polkavm::ExecutionError::Trap(_)) => RiscvExecOutcome::Trap,
			Err(err) => {
				log::error!("polkavm execution error: {}", err);
				RiscvExecOutcome::Trap
			},
		};

		Ok(outcome as u8)
	}

	fn read_memory(
		&mut self,
		offset: u32,
		buf_ptr: u32,
		buf_len: u32,
	) -> sp_wasm_interface::Result<bool> {
		let instance =
			self.riscv_data_mut().ok_or("No riscv instance found")?.caller_mut();
		let Ok(buf) = instance.read_memory_into_new_vec(offset, buf_len) else {
			return Ok(false);
		};
		util::write_memory_from(&mut self.caller, buf_ptr.into(), buf.as_ref())
			.map_err(|_| "Failed to write memory from the sandboxed instance to the supervisor")?;
		Ok(true)
	}

	fn write_memory(
		&mut self,
		offset: u32,
		buf_ptr: u32,
		buf_len: u32,
	) -> sp_wasm_interface::Result<bool> {
		let instance =
			self.riscv_data_mut().ok_or("No riscv instance found")?.caller_mut();
		let mut buf = vec![0u8; buf_len as usize];
		util::read_memory_into(&self.caller, buf_ptr.into(), buf.as_mut())
			.map_err(|_| "Failed to read memory from supervisor")?;
		if instance.write_memory(offset, buf.as_ref()).is_err() {
			return Ok(false)
		}
		Ok(true)
	}
}

struct RiscvContext<'a, 'b> {
	// needed to call into the syscall handler defined in wasm inside ecall
	host: &'a mut HostContext<'b>,
	data: RiscvData,
}

struct RiscvData {
	syscall_handler: TypedFunc<(u32, u32, u32, u32, u32, u32, u32, u32), u64>,
	caller: Option<*mut dyn Any>,
	state_ptr: u32,
}

impl<'a, 'b> RiscvContext<'a, 'b> {
	fn shared_state_mut(&mut self) -> Option<&mut RiscvState<()>> {
		let offset = self.data.state_ptr as usize;
		let buf = self
			.host
			.caller
			.as_context()
			.data()
			.memory()
			.data_mut(&mut self.host.caller);
		let scoped = buf.get_mut(offset..offset.saturating_add(mem::size_of::<RiscvState<()>>()))?;
		unsafe { Some(&mut *(scoped.as_mut_ptr() as *mut _)) }
	}
}

impl RiscvData {
	fn caller_mut(&mut self) -> &mut polkavm::Caller<'static, RiscvContext<'static, 'static>> {
		let caller = self.caller.expect("Is always set by ecall. This function can only be called during ecall; qed");
		let caller = unsafe {
			&mut *caller
		};
		caller.downcast_mut().expect("We always put the correct type there in this file; qed")
	}
}

fn on_ecall(mut riscv_caller: polkavm::Caller<'_, RiscvContext<'_, '_>>, syscall_no: u32) -> Result<(), polkavm::Trap> {
	use polkavm::Reg;

	let syscall_handler = riscv_caller.data().data.syscall_handler;
	let state_ptr = riscv_caller.data().data.state_ptr;
	let a0 = riscv_caller.get_reg(Reg::A0);
	let a1 = riscv_caller.get_reg(Reg::A1);
	let a2 = riscv_caller.get_reg(Reg::A2);
	let a3 = riscv_caller.get_reg(Reg::A3);
	let a4 = riscv_caller.get_reg(Reg::A4);
	let a5 = riscv_caller.get_reg(Reg::A5);

	// store pointers for access by read/write memory host functions during a syscall
	riscv_caller.data_mut().host
		.caller
		.data_mut()
		.host_state_mut()
		.unwrap()
		.riscv_data = Some(&mut riscv_caller.data_mut().data as *mut _);
	let mut riscv_caller: polkavm::Caller<'static, RiscvContext<'static, 'static>> = unsafe {
		mem::transmute(riscv_caller)
	};
	riscv_caller.data_mut().data.caller = Some(&mut riscv_caller as *mut _);

	let result = syscall_handler.call(
		&mut riscv_caller.data_mut().host.caller,
		(
			state_ptr, syscall_no, a0, a1, a2, a3, a4, a5,
		),
	);

	// unset as the ptr is only valid during the syscall
	riscv_caller.data_mut().host
		.caller
		.data_mut()
		.host_state_mut()
		.unwrap()
		.riscv_data = None;
	riscv_caller.data_mut().data.caller = None;

	match result {
		Err(err) => {
			log::error!("RiscV syscall handler failed: {}", err);
			Err(polkavm::Trap::default())
		},
		Ok(_)
			if riscv_caller
				.data_mut()
				.shared_state_mut()
				.expect("shared state ptr was validated when setting up instance; qed")
				.exit => Err(polkavm::Trap::default()),
		Ok(result) => {
			riscv_caller.set_reg(Reg::A0, result as u32);
			riscv_caller.set_reg(Reg::A1, (result >> 32) as u32);
			Ok(())
		},
	}
}
