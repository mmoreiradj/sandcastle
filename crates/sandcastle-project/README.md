# Sandcastle Project

This crate is responsible for expressing and managing a Sandcastle project.

A sandcastle project is a collection of resources that are used in an environment.

The goal is to allow a user to express through yaml how their environment should be created and managed.

This includes:

- a dependency graph of resources that are used in the environment
- a way to express the desired state of the environment
- a way to test wether the environment is in the desired state
- a way to tear down the environment
- a way to express where the environment should be created
