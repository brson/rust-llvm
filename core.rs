use core;
import core::*;

enum llbool = ctypes::c_int;

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
