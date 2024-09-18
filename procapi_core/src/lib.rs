pub mod process;

/*
structure:
procapi_core (only processes logic)
    └── process
        └── platform (Spec. OS impls)
            └── linux
                └── init (modules etc)
                └── ... (modules etc)
            └── macos
                └── init (modules etc)
                └── ... (modules etc)
            └── other
                └── init
                └── ... (modules etc)
         └── state
            └── platform (Spec. OS impls)
                └── linux
                └── macos
                └── other
         └── thread (later)
            └── platform (Spec. OS impls)
                └── linux
                └── macos
                └── other
 */