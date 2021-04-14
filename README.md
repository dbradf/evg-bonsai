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

There are 2 key components to using `evg-bonsai`: pots and landscapes.

### Bonsai pots

Bonsai pots allow you to package up a piece of functionality to execute in an Evergreen project into
a single logical entity. They can be used to divide a single large Evergreen project configuration up
into multiple files or to create shared packages that can be used by multiple Evergreen projects.

### Bonsai landscapes

A bonsai landscape describes how to compose one or more bonsai pots with other evergreen configuration.
It looks similar to a standard evergreen configuration file, but will have references to bonsai pots 
within it.

## Building Bonsai pots

A YAML file is used to define a pot. The YAML should contain a `name` with a unique name for the pot and
`functions` which are a map of function names to function definitions. Each function will be available
as bonsai commands in landscapes that consume them.

```yaml
name: mypot
functions:
  func1:
    ...
```

### Bonsai pot functions

A bonsai pot function has 3 parts: a required description, a list of parameters it accepts, and list of actions
it will perform.

The `description` is required. It is meant to be used as documentation to anyone consuming this pot. It should be 
provided as a string.

The `params` are a list of parameters with a `name` and `description`. These should document any parameters that
can be provided to your function to customize its behaviour. 

The `actions` are a list of [Evergreen commands](https://github.com/evergreen-ci/evergreen/wiki/Project-Commands) 
that describe how the function should execute. They should follow the same format as 
[defining functions](https://github.com/evergreen-ci/evergreen/wiki/Project-Configuration-Files#functions) in
a standard evergreen configuration file.

```yaml
name: greetings
functions:
  hello world:
    description: Tell the world hello
    params:
      - name: person
        description: Name of person being greeted
    actions:
      - command: shell.exec
        params:
          script: |
            if [ -n "${person}" ]; then 
              echo "Hello ${person}"
            else 
              echo "Hello World"
            fi
```

### Storing Bonsai pots in Github

Bonsai pots can be stored in and consumed from github. Using github makes sharing pots between multiple projects
fairly simple. A single github repository can store multiple bonsai pots. 

In order to expose pots in a github repository, a manifest file called `bonsai.manifest.yml` needs to be added 
to the root of the repository. This file describes what pots are in the repository and where they are located.

The manifest should contain a top-level `bonsai_pots` key with a list of all pots you want to expose. Each pot
should include a `path` describing where in the repository the pot lives and a `description` which provides documentation
for what the pot is meant to do. Optionally, a `include_files` key can be provided with a list of paths in the
repository that support execution of a pot. For example, external shell script that a pot will call out to. When 
an Evergreen project config is generated, these will be copied into the `bonsai_files` directory along side the
evergreen configuration.

```yaml
bonsai_pots:
  - path: bonsai/pot1.bonsai.yml
    description: A sample bonsai pot.
    
  - path: bonsai/pot2.bonsai.yml
    description: Another sample bonsai pot, but with external files.
    include_files:
      - bonsai/pot2/useful_file_0.sh
      - bonsai/pot2/useful_file_1.sh
```

## Building Bonsai landscapes

Once you have a Bonsai pot, you can start to use it in a landscape. A landscape is defined with a YAML file.
For the most part, a landscape file will look identical to a normal Evergreen project configuration file with 
a few small differences.

* A top-level bonsai configuration section.
* References to consumed pot functions can be called out to.

### Bonsai landscape configuration

The Bonsai landscape configuration describes what pots your landscape will use and how to find them. It should
be defined at the top-level of the YAML file with the `bonsai` key and will contain a list of pot descriptors.

Each pot descriptor should have a `source`, which describes how to find the pot, and other details about its location
that are depend on the source. Currently, the supported `source`s are `local` and `github`.

#### local

`local` pots are found on the local filesystem when generating the evergreen configuration. These will typically be
used only in a single project and would live with that project. `local` sources should also include a `path` key
that defines the filesystem path to find the pot configuration.
  
```yaml
- source: local
  path: path/to/bonsai/pot.yml
```

#### github

`github` pots are retrieved from a github repository. Using github to store pots makes it easy to share pots between
different projects. `github` pots should also include the `owner` and `repo` fields to describe which github repository
contains the pot(s). You can also provide an optional `branch` or `revision` key if you want more control over what
version of the pot to use. By default, the latest version of the `master` branch will be used.

### Using Bonsai pot functions

You can call out to functions from Bonsai pots anywhere in the configuration where a function call or built-in
command is valid.

Bonsai function calls use the key `bonsai` with a string value describing the function call to make. That value should
include the name of the pot that defines the function and the name of the function to call separated by a ':'. For example,
if I wanted to execute the "hello world" function from the "greetings" pot, the call would look like 
`bonsai: greetings:hello world`.

If a function supports parameters, the `params` key can be used to specify any parameter values that should be passed
to the function. For example:

```yaml
- bonsai: greetings:hello world
  params:
    person: me
```

## Examples

You can see some examples of the different configuration files in the [samples](samples/) directory.

## Generating Evergreen configuration

Once you have a landscape defined, you can use the `evg-bonsai build` command to generate an actual Evergreen
configuration file.

```
Generate an Evergreen YAML configuration from a given bonsai source

USAGE:
    evg-bonsai build [OPTIONS] --source-file <source-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --source-file <source-file>            File to build evergreen configuration from
        --target-dir <target-dir>              Directory write generated content to [default: .]
        --target-filename <target-filename>    Filename to use for generated output [default: evergreen.yml]
```

```bash
$ evg-bonsai build --source-file my.landscape.yml
```

It will then output the Evergreen project configuration to where ever was specified (`./evergreen.yml` by default). It 
will also have copied over any files required by bonsai pots being used in the `bonsai_files` directory. Be sure to
include those if you intent to package up or move the configuration.
