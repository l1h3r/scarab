// =============================================================================
// ERC20 Configuration
// =============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
  name: &'static str,
  symbol: &'static str,
  decimals: u8,
  meta: bool,
  burn: bool,
  mint: bool,
}

impl Token {
  pub const DECIMALS: u8 = 18;

  pub const fn new(name: &'static str, symbol: &'static str) -> Self {
    Self {
      name,
      symbol,
      decimals: Self::DECIMALS,
      meta: true,
      burn: false,
      mint: false,
    }
  }

  pub const fn decimals(self, decimals: u8) -> Self {
    Self { decimals, ..self }
  }

  pub const fn meta(self, meta: bool) -> Self {
    Self { meta, ..self }
  }

  pub const fn burn(self, burn: bool) -> Self {
    Self { burn, ..self }
  }

  pub const fn mint(self, mint: bool) -> Self {
    Self { mint, ..self }
  }
}
