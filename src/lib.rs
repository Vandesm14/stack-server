#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Character {
  pub char: char,
  pub index: usize,
  pub line_index: usize,
  pub line: usize,
  pub wrapped: bool,
}

pub fn string_to_chars(string: String, max_chars: usize) -> Vec<Character> {
  let mut chars: Vec<Character> = Vec::new();
  let mut line = 0;
  let mut line_index = 0;
  let mut wrapping = false;

  let string = string.replace('\n', " \n");

  let mut new_line = move || {
    line_index = 0;
    line += 1;
  };

  let string_chars = string.chars().enumerate();
  let mut count = 0;
  for (i, char) in string_chars {
    let local_max = if wrapping { max_chars + 1 } else { max_chars };

    if char == '\n' {
      new_line();
      wrapping = false;
      continue;
    }

    if (line_index >= local_max) {
      new_line();
      wrapping = true;
    }

    chars.push(Character {
      char,
      index: count,
      line,
      line_index,
      wrapped: wrapping,
    });

    line_index += 1;
    count += 1;
  }

  chars
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum EditorMode {
  #[default]
  Edit,
  Run,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum MoveAction {
  #[default]
  Invalid,

  // Modal
  Mode,

  // Navigation
  Home,
  End,
  Left,
  Right,
  Up,
  Down,

  // Input
  Enter,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Editor {
  pub code: String,
  pub cursor: usize,
  pub mode: EditorMode,
  pub buffer: String,
}

impl Editor {
  pub fn navigate(&mut self, action: MoveAction) {
    match action {
      MoveAction::Left => self.cursor -= 1,
      MoveAction::Right => self.cursor += 1,

      _ => {}
    }
  }
}
