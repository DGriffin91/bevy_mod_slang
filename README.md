# bevy_mod_slang

WIP

[Slang](https://github.com/shader-slang/slang) shaders for Bevy.

Compilation requires [slangc](https://github.com/shader-slang/slang/releases/) to be in path

- Hot reloading
- DX12 / Vulkan / WebGPU / (WebGL TBD)

```
Error with webgl2:
Internal error in ShaderStages(FRAGMENT) shader: ERROR: 0:13: '_S1__block_0Fragment' : identifiers containing two consecutive underscores (__) are reserved as possible future keywords
```