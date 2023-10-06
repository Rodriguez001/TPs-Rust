/* fn reverse_list(&self) -> List<T> {
        let mut current = self;
        let mut result = List::Nil;

        while let List::Cons(head, tail) = current {
            result = List::Cons(head, Box::new(tail));
            current = tail;
        }

        result
    }*/

    fn nth_element(&self, index: usize) -> T {
        let result;
        let mut current = *self;
        let cmpt: usize = 1;       
        result = Self::new(T);
        let List::Cons(head, tail) = current
        if ! current.is_null() {            
            if index == 1 {
                result = head;
            }
            while let List::Cons(head, tail) = current {                
                (if cmpt == index {
                    result = head;
                })
                //result = List::Cons(head, tail);
                current = *tail;
                cmpt+=1;
            }
        } 
         *result
    }