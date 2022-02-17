//! This file should be autogenerated based on the headers created from the .edl file.

use log::*;

use sgx_types::{sgx_enclave_id_t, sgx_status_t, SgxResult};

use enclave_ffi_types::{Ctx, EnclaveBuffer, HandleResult, InitResult, QueryResult};

use crate::enclave::ENCLAVE_DOORBELL;

extern "C" {
    /// Copy a buffer into the enclave memory space, and receive an opaque pointer to it.
    pub fn ecall_allocate(
        eid: sgx_enclave_id_t,
        retval: *mut EnclaveBuffer,
        buffer: *const u8,
        length: usize,
    ) -> sgx_status_t;

    /// Trigger the init method in a wasm contract
    pub fn ecall_init(
        eid: sgx_enclave_id_t,
        retval: *mut InitResult,
        context: Ctx,
        gas_limit: u64,
        used_gas: *mut u64,
        contract: *const u8,
        contract_len: usize,
        env: *const u8,
        env_len: usize,
        msg: *const u8,
        msg_len: usize,
        sig_info: *const u8,
        sig_info_len: usize,
    ) -> sgx_status_t;

    /// Trigger a handle method in a wasm contract
    pub fn ecall_handle(
        eid: sgx_enclave_id_t,
        retval: *mut HandleResult,
        context: Ctx,
        gas_limit: u64,
        used_gas: *mut u64,
        contract: *const u8,
        contract_len: usize,
        env: *const u8,
        env_len: usize,
        msg: *const u8,
        msg_len: usize,
        sig_info: *const u8,
        sig_info_len: usize,
    ) -> sgx_status_t;
}

#[cfg(not(feature = "query-node"))]
extern "C" {
    /// Trigger a query method in a wasm contract
    pub fn ecall_query(
        eid: sgx_enclave_id_t,
        retval: *mut QueryResult,
        context: Ctx,
        gas_limit: u64,
        used_gas: *mut u64,
        contract: *const u8,
        contract_len: usize,
        env: *const u8,
        env_len: usize,
        msg: *const u8,
        msg_len: usize,
    ) -> sgx_status_t;
}

#[cfg(feature = "query-node")]
extern "C" {
    /// Copy a buffer into the enclave memory space, and receive an opaque pointer to it.
    pub fn ecall_allocate_qe(
        eid: sgx_enclave_id_t,
        retval: *mut EnclaveBuffer,
        buffer: *const u8,
        length: usize,
    ) -> sgx_status_t;

    /// Trigger a query method in a wasm contract
    pub fn ecall_query_qe(
        eid: sgx_enclave_id_t,
        retval: *mut QueryResult,
        context: Ctx,
        gas_limit: u64,
        used_gas: *mut u64,
        contract: *const u8,
        contract_len: usize,
        env: *const u8,
        env_len: usize,
        msg: *const u8,
        msg_len: usize,
    ) -> sgx_status_t;
}

/// Alias for ecall_query_qe
#[cfg(feature = "query-node")]
#[allow(non_upper_case_globals)]
pub const ecall_query: unsafe extern "C" fn(
    eid: sgx_enclave_id_t,
    retval: *mut QueryResult,
    context: Ctx,
    gas_limit: u64,
    used_gas: *mut u64,
    contract: *const u8,
    contract_len: usize,
    env: *const u8,
    env_len: usize,
    msg: *const u8,
    msg_len: usize,
) -> sgx_status_t = ecall_query_qe;

/// This is a safe wrapper for allocating buffers inside the enclave.
pub(super) fn allocate_enclave_buffer(buffer: &[u8]) -> SgxResult<EnclaveBuffer> {
    let ptr = buffer.as_ptr();
    let len = buffer.len();
    let mut enclave_buffer = EnclaveBuffer::default();

    // Bind the token to a local variable to ensure its
    // destructor runs in the end of the function
    let enclave_access_token = ENCLAVE_DOORBELL
        // This is always called from an ocall contxt
        .get_access(true)
        .ok_or(sgx_status_t::SGX_ERROR_BUSY)?;

    let enclave_id = enclave_access_token
        .expect("If we got here, surely the enclave has been loaded")
        .geteid();

    trace!(
        target: module_path!(),
        "allocate_enclave_buffer() called with len: {:?} enclave_id: {:?}",
        len,
        enclave_id,
    );

    match unsafe { ecall_allocate(enclave_id, &mut enclave_buffer, ptr, len) } {
        sgx_status_t::SGX_SUCCESS => Ok(enclave_buffer),
        failure_status => Err(failure_status),
    }
}

/// This is a safe wrapper for allocating buffers inside the query enclave.
#[cfg(feature = "query-node")]
pub(super) fn allocate_enclave_buffer_qe(buffer: &[u8]) -> SgxResult<EnclaveBuffer> {
    use crate::enclave::QUERY_ENCLAVE_DOORBELL;

    let ptr = buffer.as_ptr();
    let len = buffer.len();
    let mut enclave_buffer = EnclaveBuffer::default();

    // Bind the token to a local variable to ensure its
    // destructor runs in the end of the function
    let enclave_access_token = QUERY_ENCLAVE_DOORBELL
        // This is always called from an ocall contxt
        .get_access(true)
        .ok_or(sgx_status_t::SGX_ERROR_BUSY)?;

    let enclave_id = enclave_access_token
        .expect("If we got here, surely the enclave has been loaded")
        .geteid();

    trace!(
        target: module_path!(),
        "allocate_enclave_buffer() called with len: {:?} enclave_id: {:?}",
        len,
        enclave_id,
    );

    match unsafe { ecall_allocate_qe(enclave_id, &mut enclave_buffer, ptr, len) } {
        sgx_status_t::SGX_SUCCESS => Ok(enclave_buffer),
        failure_status => Err(failure_status),
    }
}
