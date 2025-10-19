pub struct StringIter<'a>{
    byte_arr: &'a [u8],
    idx: usize,
    end_idx: usize,
}

impl StringIter<'_>{
    fn char_at(&self, idx: usize) -> char{
        self.byte_arr[idx] as char
    }

    fn first(&self) -> char{
        self.byte_arr[0] as char
    }

    fn end(&self) -> char{
        self.byte_arr[self.end_idx] as char
    }
}

impl<'a> Iterator for StringIter<'a>{
    type Item = char;

    fn next(&mut self) -> Option<char>{
        if self.idx >= self.end_idx{
            None
        } else{
            let c = self.byte_arr[self.idx] as char;
            self.idx += 1;
            Some(c)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>){
        let len = self.end_idx - self.idx;
        (len, Some(len))
    }
}

impl ExactSizeIterator for StringIter<'_>{}

impl DoubleEndedIterator for StringIter<'_>{
    fn next_back(&mut self) -> Option<char>{
        if self.end_idx > 0{
            self.end_idx -= 1;
            Some(self.byte_arr[self.end_idx] as char)
        } else{
            None
        }
    }
}

pub trait StringIndexTrait{
    fn iter(&self) -> StringIter;

    fn char_at(&self, idx: usize) -> char{
        self.iter().char_at(idx)
    }

    fn first(&self) -> char{
        self.iter().first()
    }

    fn end(&self) -> char{
        self.iter().end()
    }
}

impl StringIndexTrait for String{
    fn iter(&self) -> StringIter{
        let byte_arr = self.as_bytes();
        StringIter{
            byte_arr,
            idx: 0,
            end_idx: byte_arr.len(),
        }
    }
}

impl StringIndexTrait for &str{
    fn iter(&self) -> StringIter{
        let byte_arr = self.as_bytes();
        StringIter{
            byte_arr,
            idx: 0,
            end_idx: byte_arr.len(),
        }
    }
}