# bevy_mod_slang

WIP

[Slang](https://github.com/shader-slang/slang) shaders for Bevy.

Compilation requires [slangc](https://github.com/shader-slang/slang/releases/) to be in path

- Hot reloading
- DX12 / Vulkan / WebGPU / (WebGL TBD)

Slang has a lot in common with hlsl. It supports most hlsl features/syntax (though less so from hlsl 2021), while adding many other features on top. 
This plugin outputs Slang to SPIR-V. Here's some useful resources for using hlsl with SPIR-V output:
https://github.com/microsoft/DirectXShaderCompiler/blob/main/docs/SPIR-V.rst
https://github.com/microsoft/DirectXShaderCompiler/blob/main/docs/SPIRV-Cookbook.rst

```
Error with webgl2:
Internal error in ShaderStages(FRAGMENT) shader: ERROR: 0:13: '_S1__block_0Fragment' : identifiers containing two consecutive underscores (__) are reserved as possible future keywords
```