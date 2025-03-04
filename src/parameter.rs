use crate::common::*;

/// A single function parameter
#[derive(PartialEq, Debug)]
pub(crate) struct Parameter<'src> {
  /// The parameter name
  pub(crate) name:     Name<'src>,
  /// Parameter is variadic
  pub(crate) variadic: bool,
  /// An optional default expression
  pub(crate) default:  Option<Expression<'src>>,
}

impl<'src> Display for Parameter<'src> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    let color = Color::fmt(f);
    if self.variadic {
      write!(f, "{}", color.annotation().paint("+"))?;
    }
    write!(f, "{}", color.parameter().paint(self.name.lexeme()))?;
    if let Some(ref default) = self.default {
      write!(f, "={}", color.string().paint(&default.to_string()))?;
    }
    Ok(())
  }
}
