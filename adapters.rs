use core;
import core::*;

iface adapter<T> {
    fn adapt() -> T;
}

impl of adapter<str::sbuf> for *ctypes::c_char {
    fn adapt() -> str::sbuf unsafe {
        unsafe::reinterpret_cast(self)
    }
}

impl of adapter<*ctypes::c_char> for str::sbuf {
    fn adapt() -> *ctypes::c_char unsafe {
        unsafe::reinterpret_cast(self)
    }
}

impl of adapter<rllvm::llvm::ModuleRef> for ::core::module_ref {
    fn adapt() -> rllvm::llvm::ModuleRef unsafe {
        unsafe::reinterpret_cast(self)
    }
}

impl of adapter<::core::module_ref> for rllvm::llvm::ModuleRef {
    fn adapt() -> ::core::module_ref unsafe {
        unsafe::reinterpret_cast(self)
    }
}

impl of adapter<rllvm::llvm::ContextRef> for ::core::context_ref {
    fn adapt() -> rllvm::llvm::ContextRef unsafe {
        unsafe::reinterpret_cast(self)
    }
}

impl of adapter<::core::context_ref> for rllvm::llvm::ContextRef {
    fn adapt() -> ::core::context_ref unsafe {
        unsafe::reinterpret_cast(self)
    }
}

impl of adapter<rllvm::llvm::TypeRef> for ::core::type_ref {
    fn adapt() -> rllvm::llvm::TypeRef unsafe {
        unsafe::reinterpret_cast(self)
    }
}

impl of adapter<::core::type_ref> for rllvm::llvm::TypeRef {
    fn adapt() -> ::core::type_ref unsafe {
        unsafe::reinterpret_cast(self)
    }
}

impl of adapter<::core::type_kind> for ctypes::c_int {
    fn adapt() -> ::core::type_kind unsafe {
        unsafe::reinterpret_cast(self as ctypes::enum)
    }
}