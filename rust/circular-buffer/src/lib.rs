
pub struct CircularBuffer<T> {
    // This field is here to make the template compile and not to
    // complain about unused type parameter 'T'. Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    _capacity: usize,
    _head: usize,
    _tail: usize,
    _data: Vec<Option<T>>,
    _size: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        match capacity {
            0 => CircularBuffer::empty_buffer(),
            _ => CircularBuffer {
                _capacity: capacity,
                _head: 0,
                _tail: capacity - 1,
                _data: Vec::with_capacity(capacity),
                _size: 0
            }
        }
    }

    pub fn empty_buffer() -> Self {
        CircularBuffer {
            _capacity: 0,
            _head: 0,
            _tail: 0,
            _size: 0,
            _data: Vec::new()
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self._size == self._capacity {
            println!("the buffer is full!");
            Err(Error::FullBuffer)
        }else {
            self._tail = (self._tail + 1) % self._capacity;
            if self._data.len() < self._capacity {
                self._data.push(Some(_element));
            }else {
                self._data[self._tail] = Some(_element);
            }
            self._size += 1;
            println!("write successful!");
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self._size == 0 {
            println!("the buffer is empty!");
            Err(Error::EmptyBuffer)
        }else {
            let idx = self._head;
            self._head = (self._head + 1) % self._capacity;
            self._size -= 1;
            println!("read successful!");
            let got = std::mem::replace(&mut self._data[idx], None);
            Ok(got.unwrap())
        }
    }

    pub fn clear(&mut self) {
        while self.read().is_ok(){
            println!("clearing...");
        }
    }

    pub fn overwrite(&mut self, _element: T) {
        if self._capacity > 0 {
            self._tail = (self._tail + 1) % self._capacity;
            if self._data.len() < self._capacity {
                self._data.push(Some(_element));
            }else {
                self._data[self._tail] = Some(_element);
            }
            if self._capacity != self._size {
                self._size += 1;
            }else {
                self._head = (self._head + 1) % self._capacity;
            }
        }
    }
}
