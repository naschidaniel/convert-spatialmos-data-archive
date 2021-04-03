![[audit]](https://github.com/naschidaniel/convert-spatialmos-data-archive/actions/workflows/audit.yml/badge.svg?name=check) 
![[check]](https://github.com/naschidaniel/convert-spatialmos-data-archive/actions/workflows/check.yml/badge.svg?name=check) 
![[lint]](https://github.com/naschidaniel/convert-spatialmos-data-archive/actions/workflows/lint.yml/badge.svg?name=lint) 
![[test]](https://github.com/naschidaniel/convert-spatialmos-data-archive/actions/workflows/test.yml/badge.svg?name=test)

# Convert-SpatialMOS-data-archive 

## Purpose

Convert archived data from the data providers `zamg`, `lwd` and `gefs` into the new spatialMOS format.

## How To

The archive folder must be placed in `./data`.

```
cargo run zamg 2020
```