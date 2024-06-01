#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Character {
  pub char: char,
  pub index: usize,
  pub line_index: usize,
  pub line: usize,
  pub wrapped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Characters {
  pub chars: Vec<Character>,
}

impl Characters {
  pub fn new(chars: Vec<Character>) -> Self {
    Self { chars }
  }

  pub fn from_string(string: &str, max_chars: usize) -> Self {
    let mut chars: Vec<Character> = Vec::new();
    let mut line = 0;
    let mut line_index = 0;
    let mut wrapping = false;

    let mut count = 0;
    for char in string.chars() {
      let local_max = if wrapping { max_chars + 1 } else { max_chars };

      if char == '\n' {
        line_index = 0;
        line += 1;
        wrapping = false;
        continue;
      }

      if line_index >= local_max {
        line_index = 0;
        line += 1;
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

    Self::new(chars)
  }

  pub fn char_at_index(&self, index: usize) -> Option<&Character> {
    self.chars.iter().find(|char| char.index == index)
  }

  pub fn char_at_line_start(&self, line: usize) -> Option<&Character> {
    self.chars.iter().find(|char| char.line == line)
  }

  pub fn char_at_line_end(&self, line: usize) -> Option<&Character> {
    self.chars.iter().rev().find(|char| char.line == line)
  }

  pub fn iter(&self) -> core::slice::Iter<Character> {
    self.chars.iter()
  }
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
  pub cursor: usize,
  pub line_offset: usize,

  pub code: String,
  pub chars: Characters,
  pub buffer: String,

  pub mode: EditorMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SetCursor {
  Set,
  Increment,
  Decrement,
}

impl Editor {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }

  pub fn with_code(mut self, code: String) -> Self {
    self.code = code;
    self.refresh_chars();
    self
  }

  pub fn set_cursor(&mut self, action: SetCursor, cursor: usize) {
    let max = self.buffer.len() - 2;

    match action {
      SetCursor::Set => self.cursor = cursor.clamp(0, max),
      SetCursor::Increment => self.cursor = (self.cursor + 1).min(max),
      SetCursor::Decrement => self.cursor = self.cursor.saturating_sub(cursor),
    }
  }

  pub fn navigate(&mut self, action: MoveAction) {
    let current_char = self.chars.char_at_index(self.cursor);
    let current_line = current_char.map(|char| char.line).unwrap_or(0);

    match action {
      MoveAction::Left => self.set_cursor(SetCursor::Decrement, 1),
      MoveAction::Right => self.set_cursor(SetCursor::Increment, 1),

      MoveAction::Up => {
        let next_line = self
          .chars
          .char_at_line_start(current_line.saturating_sub(1));
        let next_line_end =
          self.chars.char_at_line_end(current_line.saturating_sub(1));

        if let (Some(current_char), Some(next_line), Some(next_line_end)) =
          (current_char, next_line, next_line_end)
        {
          self.set_cursor(
            SetCursor::Set,
            (next_line.index + current_char.line_index)
              .min(next_line_end.index),
          );
        } else {
          self.cursor = 0;
        }
      }
      MoveAction::Down => {
        let next_line = self.chars.char_at_line_start(current_line + 1);
        let next_line_end = self.chars.char_at_line_end(current_line + 1);

        if let (Some(current_char), Some(next_line), Some(next_line_end)) =
          (current_char, next_line, next_line_end)
        {
          self.set_cursor(
            SetCursor::Set,
            (next_line.index + current_char.line_index)
              .min(next_line_end.index),
          );
        } else {
          self.set_cursor(SetCursor::Set, self.buffer.len());
        }
      }

      _ => {}
    };

    let new_current_char = self.chars.char_at_index(self.cursor);
    let new_current_line = new_current_char.map(|char| char.line).unwrap_or(0);

    if new_current_line != current_line {
      if new_current_line < self.line_offset {
        // Cursor decreased
        self.line_offset = new_current_line.max(0);
      } else if new_current_line >= self.line_offset + 7 {
        // Cursor increased
        self.line_offset = new_current_line.saturating_sub(6);
      }
    }
  }

  pub fn refresh_chars(&mut self) {
    self.buffer = self.code.replace('\n', " \n");

    if !self.buffer.ends_with(' ') {
      self.buffer.push(' ');
    }

    self.chars = Characters::from_string(&self.buffer, 15);
  }

  pub fn chars_window(&self) -> std::iter::Skip<std::slice::Iter<Character>> {
    let start = self
      .chars
      .char_at_line_start(self.line_offset)
      .map(|char| char.index)
      .unwrap_or(0);
    self.chars.chars.iter().skip(start)
  }
}
