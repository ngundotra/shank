#[derive(ShankInstruction)]
pub enum Instruction {
    #[account(0, name = "creator", sig)]
    CloseThing(COption<u8>, ComplexArgs, ComplexArgs),
}
