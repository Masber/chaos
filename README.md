# CHAOS

Repo to experiment with crazy rust stuff.

## TODO

1. ~Create streams~
1. ~Create logger~
1. ~Create static logger~
1. ~Create dynamic logger~
1. ~Make logger async~
1. ~Organise code in modules~
1. ~Dynamic dispatch any type~
1. ~Create a separate module for Stream~
1. ~Reorganize Stream modules into submodules for static and dynamic dispatch~ Not needed because rust compiler can coerce from impl `Stream<Item=T>` to dyn `Stream<Item=T>`
1. Migrate main from 'futures' to 'tokio'
1. Introduce macros in main
