# Extensible Concurrency with the Send and Sync Traits

## The Send Trait
- indicates that ownership of the type implementing it can be transferred between threads
- `Rc<T>` does not implement `Send`
- `Arc<T>` does implement `Send`

## The Sync Trait
- indicates that the sharing of references of type implementing it is safe between threads
- `Rc<T>` does not implement `Sync`
- `RefCell<T>` does not implement `Sync`
- `Mutex<T>` does implement `Sync`