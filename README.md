This crate allows one to get ownership of dropped data\
Note: this crate supports no_std

**Example**
```rust
use owned_drop::OwnedDroppable;
use owned_drop::DropOwned;

fn do_stuff(owner: OwnerOfVec) {
    let mut drop = DropOwned::new(owner);
    
    // ...
    
    // `drop` gets dropped here and `drop_owned` gets called.
}

struct OwnerOfVec {
    data: Vec<String>,
}

impl OwnedDroppable for OwnerOfVec {
    fn drop_owned(self) {
        // This avoids a clone call one would normally have to do
        // if one only had `&mut self` instead if `self`
        function_requiring_ownership(self.data);
    }
}

fn function_requiring_ownership(data: Vec<String>) {
    /*
    ...
     */
}

```