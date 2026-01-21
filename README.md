# uksi
Basic types and Postgres migrations for working with the [UK Species Inventory](https://www.nhm.ac.uk/our-science/data/uk-species/index). Currently very work in progress.

## Features
- `updater`: Enabled by default. Provides support for updating Postgres from an MS Access `.mdb` file, as provided by the Natural History Museum. Includes a small number of 'corrections' to the original data that allows importing into a system with Foreign Key constraints, and strict string length and typing requirements etc. Last tested against UKSI version: 2025-11-04.
