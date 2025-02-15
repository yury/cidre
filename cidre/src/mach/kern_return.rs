use crate::os;

pub type KernReturn = os::Status;

pub mod err {
    use crate::os::Error;

    /// Specified address is not currently valid.
    #[doc(alias = "KERN_INVALID_ADDRESS")]
    pub const INVALID_ADDRESS: Error = Error::new_unchecked(1);
    ///
    /// Specified memory is valid, but does not permit the required forms of access.
    #[doc(alias = "KERN_PROTECTION_FAILURE")]
    pub const PROTECTION_FAILURE: Error = Error::new_unchecked(2);

    /// The address range specified is already in use, or no address range of the size specified could be found.
    #[doc(alias = "KERN_NO_SPACE")]
    pub const NO_SPACE: Error = Error::new_unchecked(3);

    /// The function requested was not applicable to this type of argument, or an argument is invalid
    #[doc(alias = "KERN_INVALID_ARGUMENT")]
    pub const INVALID_ARGUMENT: Error = Error::new_unchecked(4);

    /// The function could not be performed.  A catch-all.
    #[doc(alias = "KERN_FAILURE")]
    pub const FAILURE: Error = Error::new_unchecked(5);

    /// A system resource could not be allocated to fulfill
    /// this request. This failure may not be permanent.
    #[doc(alias = "KERN_RESOURCE_SHORTAGE")]
    pub const RESOURCE_SHORTAGE: Error = Error::new_unchecked(6);

    /// The task in question does not hold receive rights for the port argument.
    #[doc(alias = "KERN_NOT_RECEIVER")]
    pub const NOT_RECEIVER: Error = Error::new_unchecked(7);

    /// Bogus access restriction.
    #[doc(alias = "KERN_NO_ACCESS")]
    pub const NO_ACCESS: Error = Error::new_unchecked(8);

    /// During a page fault, the target address refers to a
    /// memory object that has been destroyed.  This
    /// failure is permanent.
    #[doc(alias = "KERN_MEMORY_FAILURE")]
    pub const MEMORY_FAILURE: Error = Error::new_unchecked(9);

    /// During a page fault, the memory object indicated
    /// that the data could not be returned.  This failure
    ///  may be temporary; future attempts to access this
    /// same data may succeed, as defined by the memory
    /// object.
    #[doc(alias = "KERN_MEMORY_ERROR")]
    pub const MEMORY_ERROR: Error = Error::new_unchecked(10);

    /// The receive right is already a member of the portset.
    #[doc(alias = "KERN_ALREADY_IN_SET")]
    pub const ALREADY_IN_SET: Error = Error::new_unchecked(11);

    /// The receive right is not a member of a port set.
    #[doc(alias = "KERN_NOT_IN_SET")]
    pub const NOT_IN_SET: Error = Error::new_unchecked(12);

    /// The name already denotes a right in the task.
    #[doc(alias = "KERN_NAME_EXISTS")]
    pub const NAME_EXISTS: Error = Error::new_unchecked(13);

    /// The operation was aborted.  Ipc code will
    /// catch this and reflect it as a message error.
    #[doc(alias = "KERN_ABORTED")]
    pub const ABORTED: Error = Error::new_unchecked(14);

    /// The name doesn't denote a right in the task.
    #[doc(alias = "KERN_INVALID_NAME")]
    pub const INVALID_NAME: Error = Error::new_unchecked(15);

    /// Target task isn't an active task.
    #[doc(alias = "KERN_INVALID_TASK")]
    pub const INVALID_TASK: Error = Error::new_unchecked(16);

    /// The name denotes a right, but not an appropriate right.
    #[doc(alias = "KERN_INVALID_RIGHT")]
    pub const INVALID_RIGHT: Error = Error::new_unchecked(17);

    /// A blatant range error.
    #[doc(alias = "KERN_INVALID_VALUE")]
    pub const INVALID_VALUE: Error = Error::new_unchecked(18);

    /// Operation would overflow limit on user-references.
    #[doc(alias = "KERN_UREFS_OVERFLOW")]
    pub const UREFS_OVERFLOW: Error = Error::new_unchecked(19);

    /// The supplied (port) capability is improper.
    #[doc(alias = "KERN_INVALID_CAPABILITY")]
    pub const INVALID_CAPABILITY: Error = Error::new_unchecked(20);

    /// The task already has send or receive rights
    /// for the port under another name.
    #[doc(alias = "KERN_RIGHT_EXISTS")]
    pub const RIGHT_EXISTS: Error = Error::new_unchecked(21);

    /// Target host isn't actually a host.
    #[doc(alias = "KERN_INVALID_HOST")]
    pub const INVALID_HOST: Error = Error::new_unchecked(22);

    /// An attempt was made to supply "precious" data
    /// for memory that is already present in a
    /// memory object.
    #[doc(alias = "KERN_MEMORY_PRESENT")]
    pub const MEMORY_PRESENT: Error = Error::new_unchecked(23);

    /// A page was requested of a memory manager via
    /// memory_object_data_request for an object using
    /// a MEMORY_OBJECT_COPY_CALL strategy, with the
    /// VM_PROT_WANTS_COPY flag being used to specify
    /// that the page desired is for a copy of the
    /// object, and the memory manager has detected
    /// the page was pushed into a copy of the object
    /// while the kernel was walking the shadow chain
    /// from the copy to the object. This error code
    /// is delivered via memory_object_data_error
    /// and is handled by the kernel (it forces the
    /// kernel to restart the fault). It will not be
    /// seen by users.
    #[doc(alias = "KERN_MEMORY_DATA_MOVED")]
    pub const MEMORY_DATA_MOVED: Error = Error::new_unchecked(24);

    /// A strategic copy was attempted of an object
    /// upon which a quicker copy is now possible.
    /// The caller should retry the copy using
    /// vm_object_copy_quickly. This error code
    /// is seen only by the kernel.
    #[doc(alias = "KERN_MEMORY_RESTART_COPY")]
    pub const MEMORY_RESTART_COPY: Error = Error::new_unchecked(25);

    /// An argument applied to assert processor set privilege
    /// was not a processor set control port.
    #[doc(alias = "KERN_INVALID_PROCESSOR_SET")]
    pub const INVALID_PROCESSOR_SET: Error = Error::new_unchecked(26);

    /// The specified scheduling attributes exceed the thread's limits.
    #[doc(alias = "KERN_POLICY_LIMIT")]
    pub const POLICY_LIMIT: Error = Error::new_unchecked(27);

    /// The specified scheduling policy is not currently
    /// enabled for the processor set.
    #[doc(alias = "KERN_INVALID_POLICY")]
    pub const INVALID_POLICY: Error = Error::new_unchecked(28);

    /// The external memory manager failed to initialize the
    /// memory object.
    #[doc(alias = "KERN_INVALID_OBJECT")]
    pub const INVALID_OBJECT: Error = Error::new_unchecked(29);

    /// A thread is attempting to wait for an event for which
    /// there is already a waiting thread.
    #[doc(alias = "KERN_ALREADY_WAITING")]
    pub const ALREADY_WAITING: Error = Error::new_unchecked(30);

    /// An attempt was made to destroy the default processor set.
    #[doc(alias = "KERN_DEFAULT_SET")]
    pub const DEFAULT_SET: Error = Error::new_unchecked(31);

    /// An attempt was made to fetch an exception port that is
    /// protected, or to abort a thread while processing a
    /// protected exception.
    #[doc(alias = "KERN_EXCEPTION_PROTECTED")]
    pub const EXCEPTION_PROTECTED: Error = Error::new_unchecked(32);

    /// A ledger was required but not supplied.
    #[doc(alias = "KERN_INVALID_LEDGER")]
    pub const INVALID_LEDGER: Error = Error::new_unchecked(33);

    /// The port was not a memory cache control port.
    #[doc(alias = "KERN_INVALID_MEMORY_CONTROL")]
    pub const INVALID_MEMORY_CONTROL: Error = Error::new_unchecked(34);

    /// An argument supplied to assert security privilege was not a host security port.
    #[doc(alias = "KERN_INVALID_SECURITY")]
    pub const INVALID_SECURITY: Error = Error::new_unchecked(35);

    /// thread_depress_abort was called on a thread which was not currently depressed.
    #[doc(alias = "KERN_NOT_DEPRESSED")]
    pub const NOT_DEPRESSED: Error = Error::new_unchecked(36);

    /// Object has been terminated and is no longer available
    #[doc(alias = "KERN_TERMINATED")]
    pub const TERMINATED: Error = Error::new_unchecked(37);

    /// Lock set has been destroyed and is no longer available.
    #[doc(alias = "KERN_LOCK_SET_DESTROYED")]
    pub const LOCK_SET_DESTROYED: Error = Error::new_unchecked(38);

    /// The thread holding the lock terminated before releasing the lock
    #[doc(alias = "KERN_LOCK_UNSTABLE")]
    pub const LOCK_UNSTABLE: Error = Error::new_unchecked(39);

    /// The lock is already owned by another thread
    #[doc(alias = "KERN_LOCK_OWNED")]
    pub const LOCK_OWNED: Error = Error::new_unchecked(40);

    /// The lock is already owned by the calling thread
    #[doc(alias = "KERN_LOCK_OWNED_SELF")]
    pub const LOCK_OWNED_SELF: Error = Error::new_unchecked(41);

    /// Semaphore has been destroyed and is no longer available.
    #[doc(alias = "KERN_SEMAPHORE_DESTROYED")]
    pub const SEMAPHORE_DESTROYED: Error = Error::new_unchecked(42);

    /// Return from RPC indicating the target server was
    /// terminated before it successfully replied
    #[doc(alias = "KERN_RPC_SERVER_TERMINATED")]
    pub const RPC_SERVER_TERMINATED: Error = Error::new_unchecked(43);

    /// Terminate an orphaned activation.
    #[doc(alias = "KERN_RPC_TERMINATE_ORPHAN")]
    pub const RPC_TERMINATE_ORPHAN: Error = Error::new_unchecked(44);

    /// Allow an orphaned activation to continue executing.
    #[doc(alias = "KERN_RPC_CONTINUE_ORPHAN")]
    pub const RPC_CONTINUE_ORPHAN: Error = Error::new_unchecked(45);

    /// Empty thread activation (No thread linked to it)
    #[doc(alias = "KERN_NOT_SUPPORTED")]
    pub const NOT_SUPPORTED: Error = Error::new_unchecked(46);

    /// Remote node down or inaccessible.
    #[doc(alias = "KERN_NODE_DOWN")]
    pub const NODE_DOWN: Error = Error::new_unchecked(47);

    /// A signalled thread was not actually waiting.
    #[doc(alias = "KERN_NOT_WAITING")]
    pub const NOT_WAITING: Error = Error::new_unchecked(48);

    /// Some thread-oriented operation (semaphore_wait) timed out
    #[doc(alias = "KERN_OPERATION_TIMED_OUT")]
    pub const OPERATION_TIMED_OUT: Error = Error::new_unchecked(49);

    /// During a page fault, indicates that the page was rejected as a result of a signature check.
    #[doc(alias = "KERN_CODESIGN_ERROR")]
    pub const CODESIGN_ERROR: Error = Error::new_unchecked(50);

    /// The requested property cannot be changed at this time.
    #[doc(alias = "KERN_POLICY_STATIC")]
    pub const POLICY_STATIC: Error = Error::new_unchecked(51);

    /// The provided buffer is of insufficient size for the requested data.
    #[doc(alias = "KERN_INSUFFICIENT_BUFFER_SIZE")]
    pub const INSUFFICIENT_BUF_SIZE: Error = Error::new_unchecked(52);

    /// Denied by security policy
    #[doc(alias = "KERN_DENIED")]
    pub const DENIED: Error = Error::new_unchecked(53);

    /// The KC on which the function is operating is missing
    #[doc(alias = "KERN_MISSING_KC")]
    pub const MISSING_KC: Error = Error::new_unchecked(54);

    /// The KC on which the function is operating is invalid
    #[doc(alias = "KERN_INVALID_KC")]
    pub const INVALID_KC: Error = Error::new_unchecked(55);

    /// A search or query operation did not return a result
    #[doc(alias = "KERN_NOT_FOUND")]
    pub const NOT_FOUND: Error = Error::new_unchecked(56);

    /// Maximum return value allowable
    #[doc(alias = "KERN_RETURN_MAX")]
    pub const RETURN_MAX: Error = Error::new_unchecked(0x100);
}
