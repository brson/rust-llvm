enum LLVMBool = ctypes::c_int;

enum LLVMContextRef = *ctypes::void;
enum LLVMModuleRef = *ctypes::void;
enum LLVMTypeRef = *ctypes::void;
enum LLVMValueRef = *ctypes::void;
enum LLVMBasicBlockRef = *ctypes::void;
enum LLVMBuilderRef = *ctypes::void;
enum LLVMModuleProviderRef = *ctypes::void;
enum LLVMMemoryBufferRef = *ctypes::void;
enum LLVMPassManagerRef = *ctypes::void;
enum LLVMPassRegistryRef = *ctypes::void;
enum LLVMUseRef = *ctypes::void;

enum LLVMAttribute {
    // FIXME: Can't const-bitshift here
    // LLVMExtAttribute = 1<<0
    LLVMZExtAttribute = 1+0,
    LLVMSExtAttribute = 2,
    LLVMNoReturnAttribute = 4,
    LLVMInRegAttribute = 8,
    LLVMStructRetAttribute = 16
}

