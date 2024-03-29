# Essentia Alchemy Engine
Essentia is an engine to facilitate alchemical transformations inspired by real-world chemistry. At it's core, its an engine that can, given a collection of inputs and description of the environment, simulate the alchemical reaction which produce outputs and modify the environment in some way.

## Abstractions
### Substance
A core abstraction of the engine. Mixing substances together, they might transform into other substances with other potential side-effects.

#### Composition
Substances are comprised of two elements: `essense` and `form`. Possible values of those elements are defined as a part of the configuration to the simulation engine.

`essense` describes the major element that makes up the substance. Examples might include `Silver` or `Water`.

`form` describes the state of the matter that makes up this substance.For example, `Crystalline` or `Fluid`.

### Solutions 
Certain forms are prone to be dissolved in other forms of matter. These rules are defined as configuraton to the simulation.

### Environment
This abstraction defines the environment under which the reaction is happening.
Certain reactions might influence the environment.

Two major elements of the environment are `temperature` and `time`, but other environment parameters might be defined for the simulation.

### Reaction
Reactions define the exact rules substances get transformed and how exactly they modify the environment they are in. 

> These reactions are made up and are only rougly inspired by the real world. 

For example, given a source of fiery magical energy, it heats up the environment over time:
```essentia

{
    Substance(Pyroflux, Salt) 
} ===(Time=10)==> {
    Substance(Pyroflux, Salt),
    Exotherm(10)
}
```

Or, given an acid mixed with a base, over time a salt solution is formed:
```essentia
{
    Substance(Vinegar, Acid),
    Substance(Lye, Base)
} ===(Time=10)==> {
    Substance(Water, Fluid) {
        Solution(Potash, Salt)
    }
}
```