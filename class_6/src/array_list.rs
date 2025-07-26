use std::mem::MaybeUninit;
use std::ptr;

pub struct ArrayList<T> {
    data: Box<[MaybeUninit<T>]>,
    length: usize,
    capacity: usize,
}

impl<T> Drop for ArrayList<T> {
    fn drop(&mut self) {
        for i in 0..self.length {
            unsafe {
                ptr::drop_in_place(self.data[i].as_mut_ptr());
            }
        }
    }
}

impl<T> ArrayList<T> {
    pub fn init() -> Self {
        let capacity = 2;
        let data = Self::allocate(capacity);
        Self {
            data,
            length: 0,
            capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.length >= self.capacity {
            self.resize();
        }

        self.data[self.length].write(value);
        self.length += 1;
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;       // allocates new capacity
        let mut new_data = Self::allocate(new_capacity); // moves element from old memory to new one

        for i in 0..self.length {
            unsafe {
                let val = self.data[i].as_ptr().read(); // extract value
                new_data[i].write(val); // store value
            }
        }

        for i in 0..self.length {
            unsafe {
                ptr::drop_in_place(self.data[i].as_mut_ptr());
            }
        }

        // drop the old memory automatically by reassigning the data
        self.data = new_data;
        self.capacity = new_capacity;
    }

    fn allocate(cap: usize) -> Box<[MaybeUninit<T>]> {
        let mut vec: Vec<MaybeUninit<T>> = Vec::with_capacity(cap);  // create a vector with reserved memory
        let ptr = vec.as_mut_ptr(); // get the pointer to the memory -> basically telling where the starting point of the vector would be
        std::mem::forget(vec);  // prevent deallocation of memory

        // wrapping the raw memory in the box to perform operations like read or write later *maybe*
        unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, cap)) }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < 0 || index >= self.length {
            None
        } else {
            unsafe { Some(&*self.data[index].as_ptr()) }
        }
    }

    pub fn size(&self) -> usize {
        self.length
    }

    pub fn print(&self)
    where
        T: std::fmt::Debug,
    {
        print!("[");
        for i in 0..self.length {
            unsafe {
                print!("{:?}", &*self.data[i].as_ptr());
                if i < self.length - 1 {
                    print!(", ");
                }
            }
        }
        println!("]");
    }
}
