// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

use core::marker::PhantomData;

use alloc::rc::Rc;

pub use unicode_linebreak::BreakOpportunity;

#[derive(Clone)]
pub struct LineBreakIterator<'a> {
    breaks: Rc<Vec<(usize, unicode_linebreak::BreakOpportunity)>>,
    pos: usize,
    phantom: PhantomData<&'a str>,
}

impl<'a> LineBreakIterator<'a> {
    pub fn new(text: &str) -> Self {
        let iterator = unicode_linebreak::linebreaks(text).filter(|(offset, opportunity)| {
            // unicode-linebreaks emits a mandatory break at the end of the text. We're not interested
            // in that.
            if *offset == text.len() && matches!(opportunity, BreakOpportunity::Mandatory) {
                false
            } else {
                true
            }
        });

        Self { breaks: Rc::new(iterator.collect()), pos: 0, phantom: Default::default() }
    }
}

impl<'a> Iterator for LineBreakIterator<'a> {
    type Item = (usize, unicode_linebreak::BreakOpportunity);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.breaks.len() {
            let i = self.pos;
            self.pos += 1;
            Some(self.breaks[i])
        } else {
            None
        }
    }
}
