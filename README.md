# How da heck do i build this shit?

You need cmake, make and g++ or clang

## Configuring

```sh
cmake -S . -B ./build
```

## Building

```sh
make -C ./build
```

## Running

```sh
./build/{proj_name}/{exec_name}
```

Profit!