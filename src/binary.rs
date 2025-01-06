pub mod module;
pub mod section;

#[cfg(test)]
mod tests {
    use crate::binary::module::Module;
    use anyhow::Result;

    #[test]
    fn decode_simplest_module() -> Result<()> {
        let wasm = wat::parse_str("(module)")?;
        let module = Module::new(&wasm)?;
        assert_eq!(module, Module::default());
        Ok(())
    }

    #[test]
    fn decode_simplest_func() -> Result<()> {
        let wasm = wat::parse_str("(module (func))")?;
        let module = Module::new(&wasm)?;
        // TODO: parse func
        assert_eq!(module, Module::default());
        Ok(())
    }    
}