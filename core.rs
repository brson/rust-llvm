use core;
import core::*;
import adapters::*;

type llbool = ctypes::c_int;

enum opaque_ref = *ctypes::void;

#[doc = "
The top-level container for all LLVM global data.  See the LLVMContext
class.
"]
enum context_ref = opaque_ref;

#[doc = "
The top-level container for all other LLVM Intermediate Representation
(IR) objects. See the llvm::Module class.
"]
enum module_ref = opaque_ref;

#[doc = "
Each value in the LLVM IR has a type, an LLVMTypeRef. See the
llvm::Type class.
"]
enum type_ref = opaque_ref;
enum value_ref = opaque_ref;
enum basic_block_ref = opaque_ref;
enum builder_ref = opaque_ref;

#[doc = "
Interface used to provide a module to JIT or interpreter.  This is now
just a synonym for llvm::Module, but we have to keep using the
different type to keep binary compatibility.
"]
enum module_provider_ref = opaque_ref;

#[doc = "
Used to provide a module to JIT or interpreter.  See the
llvm::MemoryBuffer class.
"]
enum memory_buffer_ref = opaque_ref;

#[doc = "
See the llvm::PassManagerBase class.
"]
enum pass_manager_ref = opaque_ref;

#[doc = "
See the llvm::PassRegistry class.
"]
enum pass_registry_ref = opaque_ref;

#[doc = "
Used to get the users and usees of a Value. See the llvm::Use class.
"]
enum use_ref = opaque_ref;

enum attribute {
    zext_attribute              = 1<<0,
    sext_attribute              = 1<<1,
    no_return_attribute         = 1<<2,
    in_reg_attribute            = 1<<3,
    struct_ret_attribute        = 1<<4,
    no_unwind_attribute         = 1<<5,
    no_alias_attribute          = 1<<6,
    by_val_attribute            = 1<<7,
    nest_attribute              = 1<<8,
    read_none_attribute         = 1<<9,
    read_only_attribute         = 1<<10,
    no_inline_attribute         = 1<<11,
    always_inline_attribute     = 1<<12,
    optimize_for_size_attribute = 1<<13,
    stack_protect_attribute     = 1<<14,
    stack_protect_req_attribute = 1<<15,
    alignment                   = 31<<16,
    no_capture_attribute        = 1<<21,
    no_red_zone_attribute       = 1<<22,
    no_implicit_float_attribute = 1<<23,
    naked_attribute             = 1<<24,
    inline_hint_attribute       = 1<<25,
    stack_alignment             = 7<<26,
    returns_twice               = 1<<29,
    uw_table                    = 1<<30,
    non_lazy_bind               = 1<<31,
}

#[test]
fn attribute_sanity_check() {
    assert zext_attribute as uint == rllvm::LLVMZExtAttribute;
    assert sext_attribute as uint == rllvm::LLVMSExtAttribute;
    assert no_return_attribute as uint == rllvm::LLVMNoReturnAttribute;
    assert in_reg_attribute as uint == rllvm::LLVMInRegAttribute;
    assert struct_ret_attribute as uint == rllvm::LLVMStructRetAttribute;
    assert no_unwind_attribute as uint == rllvm::LLVMNoUnwindAttribute;
    assert by_val_attribute as uint == rllvm::LLVMByValAttribute;
    assert nest_attribute as uint == rllvm::LLVMNestAttribute;
    assert read_none_attribute as uint == rllvm::LLVMReadNoneAttribute;
    assert read_only_attribute as uint == rllvm::LLVMReadOnlyAttribute;
    assert no_inline_attribute as uint == rllvm::LLVMNoInlineAttribute;
    assert always_inline_attribute as uint == rllvm::LLVMAlwaysInlineAttribute;
    assert optimize_for_size_attribute as uint == rllvm::LLVMOptimizeForSizeAttribute;
    assert stack_protect_attribute as uint == rllvm::LLVMStackProtectAttribute;
    assert alignment as uint == rllvm::LLVMAlignmentAttribute;
    assert no_capture_attribute as uint == rllvm::LLVMNoCaptureAttribute;
    assert no_red_zone_attribute as uint == rllvm::LLVMNoRedZoneAttribute;
    assert no_implicit_float_attribute as uint == rllvm::LLVMNoImplicitFloatAttribute;
    assert naked_attribute as uint == rllvm::LLVMNakedAttribute;
    assert inline_hint_attribute as uint == rllvm::LLVMInlineHintAttribute;
    assert stack_alignment as uint == rllvm::LLVMStackAttribute;
    assert returns_twice as uint == rllvm::LLVMReturnsTwiceAttribute;
    assert uw_table as uint == rllvm::LLVMUWTableAttribute;
    assert non_lazy_bind as uint == rllvm::LLVMNonLazyBindAttribute;
}

enum opcode {
    /* Terminator instructions */
    return          = 1,
    br              = 2,
    switch          = 3,
    indirectbr      = 4,
    invoke          = 5,
    unreachable     = 7,

    /* Standard binary operators */
    add             = 8,
    fadd            = 9,
    sub             = 10,
    fsub            = 11,
    mul             = 12,
    fmul            = 13,
    udiv            = 14,
    sdiv            = 15,
    fdiv            = 16,
    urem            = 17,
    srem            = 18,
    frem            = 19,

    /* Logical operators */
    shl             = 20,
    lshr            = 21,
    ashr            = 22,
    and             = 23,
    or              = 24,
    xor             = 25,

    /* Memory operators */
    alloca          = 26,
    load            = 27,
    store           = 28,
    getelementptr   = 29,

    /* Cast operators */
    trunc           = 30,
    zext            = 31,
    sext            = 32,
    fptoui          = 33,
    fptosi          = 34,
    uitofp          = 35,
    sitofp          = 36,
    fptrunc         = 37,
    fpext           = 38,
    ptrtoint        = 39,
    inttoptr        = 40,
    bitcast         = 41,

    /* Other operators */
    icmp            = 42,
    fcmp            = 43,
    phi             = 44,
    call            = 45,
    select          = 46,
    userop1         = 47,
    userop2         = 48,
    va_arg          = 49,
    extractelement  = 50,
    insertelement   = 51,
    shufflevector   = 52,
    extractvalue    = 53,
    insertvalue     = 54,

    /* Atomic operators */
    fence           = 55,
    cmpxchg         = 56,
    atomicrmw       = 57,

    /* Exception handling operators */
    resume          = 58,
    landingpad      = 59,
    unwind          = 60,
}

enum type_kind {
    #[doc = "type with no size"]
    void_type_kind,
    #[doc = "16 bit floating point type"]
    half_type_kind,
    #[doc = "32 bit floating point type"]
    float_type_kind,
    #[doc = "64 bit floating point type"]
    double_type_kind,
    #[doc = "80 bit floating point type (X87)"]
    x86_fp80_type_kind,
    #[doc = "128 bit floating point type (112-bit mantissa)"]
    fp128_type_kind,
    #[doc = "128 bit floating point type (two 64-bits)"]
    ppc_fp128_type_kind,
    #[doc = "Labels"]
    label_type_kind,
    #[doc = "Arbitrary bit width integers"]
    integer_type_kind,
    #[doc = "Functions"]
    function_type_kind,
    #[doc = "Structures"]
    struct_type_kind,
    #[doc = "Arrays"]
    array_type_kind,
    #[doc = "Pointers"]
    pointer_type_kind,
    #[doc = "SIMD 'packed' format, or other vector type"]
    vector_type_kind,
    #[doc = "Metadata"]
    metadata_type_kind,
    #[doc = "X86 MMX"]
    x86_mmx_type_kind,
}

enum linkage {
    #[doc = "Externally visible function"]
    external_linkage,
    available_externally_linkage,
    #[doc = "Keep one copy of function when linking (inline)"]
    link_once_any_linkage,
    #[doc = "Same, but only replaced by something equivalent."]
    link_once_odr_linkage,
    #[doc = "Keep one copy of function when linking (weak)"]
    weak_any_linkage,
    #[doc = "Same, but only replaced by something equivalent."]
    weak_odr_linkage,
    #[doc = "Special purpose, only applies to global arrays"]
    appending_linkage,
    #[doc = "Rename collisions when linking (static functions)"]
    internal_linkage,
    #[doc = "Like Internal, but omit from symbol table"]
    private_linkage,
    #[doc = "Function to be imported from DLL"]
    dll_import_linkage,
    #[doc = "Function to be accessible from DLL"]
    dll_export_linkage,
    external_weak_linkage,
    #[deprecated]
    ghost_linkage,
    common_linkage,
    #[doc = "Like private_linkage, but linker removes."]
    linker_private_linkage,
    #[doc = "Like linker_private_linkage, but is weak."]
    linker_private_weak_linkage,
    #[doc = "Like linker_private_weak, but possibly hidden."]
    linker_private_weak_def_auto_linkage
}

#[test]
fn linkage_sanity_check() {
    assert external_linkage as uint == rllvm::LLVMExternalLinkage;
    assert available_externally_linkage as uint == rllvm::LLVMAvailableExternallyLinkage;
    assert link_once_any_linkage as uint == rllvm::LLVMLinkOnceAnyLinkage;
    assert link_once_odr_linkage as uint == rllvm::LLVMLinkOnceODRLinkage;
    assert weak_any_linkage as uint == rllvm::LLVMWeakAnyLinkage;
    assert weak_odr_linkage as uint == rllvm::LLVMWeakODRLinkage;
    assert appending_linkage as uint == rllvm::LLVMAppendingLinkage;
    assert internal_linkage as uint == rllvm::LLVMInternalLinkage;
    assert private_linkage as uint == rllvm::LLVMPrivateLinkage;
    assert dll_import_linkage as uint == rllvm::LLVMDLLImportLinkage;
    assert dll_export_linkage as uint == rllvm::LLVMDLLExportLinkage;
    assert external_weak_linkage as uint == rllvm::LLVMExternalWeakLinkage;
    assert ghost_linkage as uint == rllvm::LLVMGhostLinkage;
    assert common_linkage as uint == rllvm::LLVMCommonLinkage;
    assert linker_private_linkage as uint == rllvm::LLVMLinkerPrivateLinkage;
    assert linker_private_weak_linkage as uint == rllvm::LLVMLinkerPrivateWeakLinkage;
    assert linker_private_weak_def_auto_linkage as uint == rllvm::LLVMLinkerPrivateWeakDefAutoLinkage;
}

enum visibility {
    #[doc = "The GV is visible"]
    default_visibility,
    #[doc = "The GV is hidden"]
    hidden_visibility,
    #[doc = "The GV is protected"]
    protected_visibility,
}

#[test]
fn visibility_sanity_check() {
    assert default_visibility as uint == rllvm::LLVMDefaultVisibility;
    assert hidden_visibility as uint == rllvm::LLVMHiddenVisibility;
    assert protected_visibility as uint == rllvm::LLVMProtectedVisibility;
}

enum call_conv {
    c_call_conv            = 0,
    fast_call_conv         = 8,
    cold_call_conv         = 9,
    x86_stdcall_call_conv  = 64,
    x86_fastcall_call_conv = 65,
}

#[test]
fn call_conv_sanity_check() {
    assert c_call_conv as uint == rllvm::LLVMCCallConv;
    assert fast_call_conv as uint == rllvm::LLVMFastCallConv;
    assert cold_call_conv as uint == rllvm::LLVMColdCallConv;
    assert x86_stdcall_call_conv as uint == rllvm::LLVMX86StdcallCallConv;
    assert x86_fastcall_call_conv as uint == rllvm::LLVMX86FastcallCallConv;
}

enum int_predicate {
    #[doc = "equal"]
    int_eq = 32,
    #[doc = "not equal"]
    int_ne,
    #[doc = "unsigned greater than"]
    int_ugt,
    #[doc = "unsigned greater or equal"]
    int_uge,
    #[doc = "unsigned less than"]
    int_ult,
    #[doc = "unsigned less or equal"]
    int_ule,
    #[doc = "signed greater than"]
    int_sgt,
    #[doc = "signed greater or equal"]
    int_sge,
    #[doc = "signed less than"]
    int_slt,
    #[doc = "signed less or equal"]
    int_sle,
}

#[test]
fn int_predicate_sanity_check() {
    assert int_eq as uint == rllvm::LLVMIntEQ;
    assert int_ne as uint == rllvm::LLVMIntNE;
    assert int_ugt as uint == rllvm::LLVMIntUGT;
    assert int_uge as uint == rllvm::LLVMIntUGE;
    assert int_ult as uint == rllvm::LLVMIntULT;
    assert int_ule as uint == rllvm::LLVMIntULE;
    assert int_sgt as uint == rllvm::LLVMIntSGT;
    assert int_sge as uint == rllvm::LLVMIntSGE;
    assert int_slt as uint == rllvm::LLVMIntSLT;
    assert int_sle as uint == rllvm::LLVMIntSLE;
}

enum real_predicate {
    #[doc = "Always false (always folded)"]
    real_predicate_false,
    #[doc = "True if ordered and equal"]
    real_oeq,
    #[doc = "True if ordered and greater than"]
    real_ogt,
    #[doc = "True if ordered and greater than or equal"]
    real_oge,
    #[doc = "True if ordered and less than"]
    real_olt,
    #[doc = "True if ordered and less than or equal"]
    real_ole,
    #[doc = "True if ordered and operands are unequal"]
    real_one,
    #[doc = "True if ordered (no nans)"]
    real_ord,
    #[doc = "True if unordered: isnan(X) | isnan(Y)"]
    real_uno,
    #[doc = "True if unordered or equal"]
    real_ueq,
    #[doc = "True if unordered or greater than"]
    real_ugt,
    #[doc = "True if unordered, greater than, or equal"]
    real_uge,
    #[doc = "True if unordered or less than"]
    real_ult,
    #[doc = "True if unordered, less than, or equal"]
    real_ule,
    #[doc = "True if unordered or not equal"]
    real_une,
    #[doc = "Always true (always folded)"]
    real_predicate_true,
}

#[test]
fn real_predicate_sanity_check() {
    assert real_oeq as uint == rllvm::LLVMRealOEQ;
    assert real_ogt as uint == rllvm::LLVMRealOGT;
    assert real_oge as uint == rllvm::LLVMRealOGE;
    assert real_olt as uint == rllvm::LLVMRealOLT;
    assert real_ole as uint == rllvm::LLVMRealOLE;
    assert real_one as uint == rllvm::LLVMRealONE;
    assert real_ord as uint == rllvm::LLVMRealORD;
    assert real_uno as uint == rllvm::LLVMRealUNO;
    assert real_ueq as uint == rllvm::LLVMRealUEQ;
    assert real_ugt as uint == rllvm::LLVMRealUGT;
    assert real_uge as uint == rllvm::LLVMRealUGE;
    assert real_ult as uint == rllvm::LLVMRealULT;
    assert real_ule as uint == rllvm::LLVMRealULE;
    assert real_une as uint == rllvm::LLVMRealUNE;
}

enum landingpad_clause_ty {
    #[doc = "A catch clause"]
    landing_pad_catch,
    #[doc = "A filter clause"]
    landing_pad_filter
}

fn initialize_core(r: pass_registry_ref) {
    rustllvm::LLVMInitializeCore(r)
}

fn dispose_message(message: *ctypes::c_char) {
    rustllvm::LLVMDisposeMessage(message)
}

fn context_create() -> context_ref {
    rllvm::llvm::LLVMContextCreate().adapt()
}

fn get_global_context() -> context_ref {
    rllvm::llvm::LLVMGetGlobalContext().adapt()
}

fn context_dispose(c: context_ref) {
    rllvm::llvm::LLVMContextDispose(c.adapt())
}

fn get_md_kind_id_in_context(
    c: context_ref,
    name: *ctypes::c_char,
    slen: ctypes::unsigned
) -> ctypes::unsigned {
    rllvm::llvm::LLVMGetMDKindIDInContext(c.adapt(), name.adapt(), slen)
}

fn get_md_kind_id(
    name: *ctypes::c_char,
    slen: ctypes::unsigned
) -> ctypes::unsigned {
    rllvm::llvm::LLVMGetMDKindID(name.adapt(), slen)
}

#[doc = "See llvm::Module::Module"]
fn module_create_with_name(module_id: *ctypes::c_char) -> module_ref {
    rustllvm::LLVMModuleCreateWithName(module_id)
}

#[doc = "See llvm::Module::Module"]
fn module_create_with_name_in_context(
    module_id: *ctypes::c_char,
    c: context_ref
) -> module_ref {
    rllvm::llvm::LLVMModuleCreateWithNameInContext(
        module_id.adapt(),
        c.adapt()
    ).adapt()
}

#[doc = "See llvm::Module::~Module"]
fn dispose_module(m: module_ref) {
    rllvm::llvm::LLVMDisposeModule(m.adapt())
}

#[doc = "See Module::getDataLayout"]
fn get_data_layout(m: module_ref) -> *ctypes::c_char {
    rllvm::llvm::LLVMGetDataLayout(m.adapt()).adapt()
}

fn set_data_layout(m: module_ref, triple: *ctypes::c_char) {
    rllvm::llvm::LLVMSetDataLayout(m.adapt(), triple.adapt())
}

#[doc = "See Module::getTargetTriple"]
fn get_target(m: module_ref) -> *ctypes::c_char {
    rllvm::llvm::LLVMGetTarget(m.adapt()).adapt()
}

fn set_target(m: module_ref, triple: *ctypes::c_char) {
    rllvm::llvm::LLVMSetTarget(m.adapt(), triple.adapt())
}

#[doc = "See Module::dump"]
fn dump_module(m: module_ref) {
    rllvm::llvm::LLVMDumpModule(m.adapt())
}

#[doc = "See Module::setModuleInlineAsm"]
fn set_module_inline_asm(m: module_ref, asm: *ctypes::c_char) {
    rllvm::llvm::LLVMSetModuleInlineAsm(m.adapt(), asm.adapt())
}

#[doc = "See Module::getContext"]
fn get_module_context(m: module_ref) -> context_ref {
    rustllvm::LLVMGetModuleContext(m)
}

#[doc = "See llvm::LLVMTypeKind::getTypeID"]
fn get_type_kind(ty: type_ref) -> type_kind {
    rllvm::llvm::LLVMGetTypeKind(ty.adapt()).adapt()
}

fn type_is_sized(ty: type_ref) -> llbool {
    rustllvm::LLVMTypeIsSized(ty)
}

#[doc = "See llvm::LLVMType::getContext"]
fn get_type_context(ty: type_ref) -> context_ref {
    rllvm::llvm::LLVMGetTypeContext(ty.adapt()).adapt()
}

fn int1_type_in_context(c: context_ref) -> type_ref {
    rllvm::llvm::LLVMInt1TypeInContext(c.adapt()).adapt()
}

fn int8_type_in_context(c: context_ref) -> type_ref {
    rllvm::llvm::LLVMInt8TypeInContext(c.adapt()).adapt()
}

fn int16_type_in_context(c: context_ref) -> type_ref {
    rllvm::llvm::LLVMInt16TypeInContext(c.adapt()).adapt()
}

fn int32_type_in_context(c: context_ref) -> type_ref {
    rllvm::llvm::LLVMInt32TypeInContext(c.adapt()).adapt()
}

fn int64_type_in_context(c: context_ref) -> type_ref {
    rllvm::llvm::LLVMInt64TypeInContext(c.adapt()).adapt()
}

fn int_type_in_context(c: context_ref, num_bits: ctypes::unsigned) -> type_ref {
    rllvm::llvm::LLVMIntTypeInContext(c.adapt(), num_bits).adapt()
}

fn int1_type() -> type_ref {
    rllvm::llvm::LLVMInt1Type().adapt()
}

fn int8_type() -> type_ref {
    rllvm::llvm::LLVMInt8Type().adapt()
}

fn int16_type() -> type_ref {
    rllvm::llvm::LLVMInt16Type().adapt()
}

fn int32_type() -> type_ref {
    rllvm::llvm::LLVMInt32Type().adapt()
}

fn int64_type() -> type_ref {
    rllvm::llvm::LLVMInt64Type().adapt()
}

fn int_type(num_bits: ctypes::unsigned) -> type_ref {
    rllvm::llvm::LLVMIntType(num_bits).adapt()
}

fn get_int_type_width(integer_ty: type_ref) -> ctypes::unsigned {
    rllvm::llvm::LLVMGetIntTypeWidth(integer_ty.adapt())
}

fn half_type_in_context(c: context_ref) -> type_ref {
    rustllvm::LLVMHalfTypeInContext(c)
}

fn float_type_in_context(c: context_ref) -> type_ref {
    rllvm::llvm::LLVMFloatTypeInContext(c.adapt()).adapt()
}

fn double_type_in_context(c: context_ref) -> type_ref {
    rllvm::llvm::LLVMDoubleTypeInContext(c.adapt()).adapt()
}

fn x86_fp80_type_in_context(c: context_ref) -> type_ref {
    rllvm::llvm::LLVMX86FP80TypeInContext(c.adapt()).adapt()
}

fn fp128_type_in_context(c: context_ref) -> type_ref {
    rllvm::llvm::LLVMFP128TypeInContext(c.adapt()).adapt()
}

fn ppc_fp128_type_in_context(c: context_ref) -> type_ref {
    rllvm::llvm::LLVMPPCFP128TypeInContext(c.adapt()).adapt()
}

fn half_type() -> type_ref {
    rustllvm::LLVMHalfType()
}

fn float_type() -> type_ref {
    rllvm::llvm::LLVMFloatType().adapt()
}

fn double_type() -> type_ref {
    rllvm::llvm::LLVMDoubleType().adapt()
}

fn x86_fp80_type() -> type_ref {
    rllvm::llvm::LLVMX86FP80Type().adapt()
}

fn fp128_type() -> type_ref {
    rllvm::llvm::LLVMFP128Type().adapt()
}

fn ppc_fp128_type() -> type_ref {
    rllvm::llvm::LLVMPPCFP128Type().adapt()
}

native mod rustllvm {
    fn LLVMInitializeCore(R: pass_registry_ref);
    fn LLVMDisposeMessage(Message: *ctypes::c_char);
    fn LLVMModuleCreateWithName(ModuleID: *ctypes::c_char) -> module_ref;
    fn LLVMGetModuleContext(M: module_ref) -> context_ref;
    fn LLVMTypeIsSized(Ty: type_ref) -> llbool;
    fn LLVMHalfTypeInContext(C: context_ref) -> type_ref;
    fn LLVMHalfType() -> type_ref;
}