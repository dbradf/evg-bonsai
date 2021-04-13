# Evergreen Bonsai

A tool for building composable [Evergreen](https://github.com/evergreen-ci/evergreen) project configurations.


                                && &&
                            &  &&&&&
                           && &&&&&&&
                          &&&&&&&|/
                            &&&&/|\&
                              &&|/    &                          ┌───────────────────┐
                              \&|\                               │ Evergreen Bonsai  │
                              \/~\                       &       └───────────────────┘
                               /~|\                &  & &  & &
                           \_\|_||\               &&&&&&&&   &
                         \    \\/~|\           & & &&&&&//&
                    &&\__        /~                  &&& &
                  &&&&_&& &&      /~|\       /   /__/
                &&&&\&& &&&        /~      __// _/
                 & &&   &           _/~   /
                                  |/~ ____/
                                 /~/~
                     :___________./~~~\.___________:
                      \                           /
                       \_________________________/
                       (_)                     (_)


## Understanding Bonsai

Evergreen provides a simple project configuration format that is flexible enough to 
pretty much let projects do whatever they need to do. The cost of this simplicity and flexibility, 
however, is that configuration can quickly grow out of control and there is no way to share
configuration between project leading to a lot of duplicate configurations.

Evergreen Bonsai is meant to address those 2 pain points. It allows you to build components to
perform actions in Evergreen and then compose those components together to compile down into
an actual Evergreen configuration.

There are 2 key components to using `evg-bonsai`: pots and landscapes

### Bonsai pots

Bonsai pots allow you to package up a piece of functionality to execute in an Evergreen project into
a single logical entity. They can be used to divide a single large Evergreen project configuration up
into multiple files or to create shared packages that can be used by multiple Evergreen projects.

### Bonsai landscapes

A bonsai landscape describes how to compose one or more bonsai pots with other evergreen configuration.
It looks similar to a standard evergreen configuration file, but will have references to bonsai pots 
within it.


