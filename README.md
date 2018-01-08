# The-Revenge-of-the-Ants-TSP-
Implementation of an ant colony in order to solve the NP-Hard TSP (Hamiltonian Path)

David Felipe Hernandez Chiapa

Second Project.
-------------------------------------------------------------

Usage:

-First of all you need to stablish de set of cities that will be in the path, and if you want to have a help-path, as well as the set of seeds for the random number generator. All of this in the Ajustes.toml.

To execute, move to the ants_tsp/ directory.

There are three ways to execute:

1) $cargo run --release

This way will execute with a random initial city.

2) $cargo run --release ciudadInicial {city_id}

This will set the city_id of the first city of the path.

3) $cargo run --release

The same as the first one but adding an initial path different from '[]' to the Ajustes.toml file.

If you try to run the second and the third way simultaneously, the thrid way will overwrite the second.

-------------------------------------------------------------

This project was tested with the next version of rustc:

rustc 1.25.0-nightly (6828cf901 2018-01-06)


The stable version of rustc is also supported.
