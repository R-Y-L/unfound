//! Unfound 过程宏 - 用于扩展 ArceOS 文件系统
//! 
//! 提供 `#[unfound_hook]` 宏,在函数执行前后自动注入 UNotify 事件触发

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Meta, NestedMeta};

/// 为函数添加 Unfound 钩子
/// 
/// 用法:
/// ```rust
/// #[unfound_hook(event = "Access", cache_action = "Read")]
/// pub fn read_file(path: &str) -> Result<Vec<u8>> {
///     // 原始实现
/// }
/// ```
/// 
/// 参数:
/// - `event`: UNotify 事件类型 (Access, Modify, Create, Delete)
/// - `path_param`: 路径参数名 (默认 "path")
#[proc_macro_attribute]
pub fn unfound_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    
    // 解析属性参数
    let attr_args = parse_macro_input!(attr as syn::AttributeArgs);
    
    let mut event_type = None;
    let mut path_param = "path".to_string();
    
    for arg in attr_args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            let ident = nv.path.get_ident().unwrap().to_string();
            if let syn::Lit::Str(lit) = nv.lit {
                match ident.as_str() {
                    "event" => event_type = Some(lit.value()),
                    "path_param" => path_param = lit.value(),
                    _ => {}
                }
            }
        }
    }
    
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;
    
    // 生成事件触发代码
    let event_trigger = if let Some(event) = event_type {
        let event_ident = syn::Ident::new(&event, proc_macro2::Span::call_site());
        let path_ident = syn::Ident::new(&path_param, proc_macro2::Span::call_site());
        quote! {
            if let Some(watcher) = unotify::get_watcher() {
                let event = unotify::NotifyEvent::new(
                    unotify::EventType::#event_ident,
                    alloc::string::ToString::to_string(#path_ident)
                );
                watcher.trigger(event);
            }
        }
    } else {
        quote! {}
    };
    
    // 重新组装函数
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            #event_trigger
            
            let result = (|| #fn_block)();
            
            result
        }
    };
    
    TokenStream::from(expanded)
}

/// 为结构体自动实现 Unfound 跟踪
/// 
/// 用法:
/// ```rust
/// #[derive(UnfoundTracked)]
/// pub struct File {
///     path: String,
///     // ...
/// }
/// ```
#[proc_macro_derive(UnfoundTracked)]
pub fn derive_unfound_tracked(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;
    
    let expanded = quote! {
        impl ::unfound_fs::Tracked for #name {
            fn on_access(&self) {
                if let Some(watcher) = ::unfound_fs::get_unotify_watcher() {
                    let event = ::unfound_fs::NotifyEvent::new(
                        ::unfound_fs::EventType::Access,
                        self.path.clone()
                    );
                    watcher.trigger(event);
                }
            }
            
            fn on_modify(&self) {
                if let Some(watcher) = ::unfound_fs::get_unotify_watcher() {
                    let event = ::unfound_fs::NotifyEvent::new(
                        ::unfound_fs::EventType::Modify,
                        self.path.clone()
                    );
                    watcher.trigger(event);
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}
