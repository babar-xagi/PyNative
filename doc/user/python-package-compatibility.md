# Python Package Compatibility

PyNative UI will use a three-level compatibility model.

| Level | Name | Use Case |
| --- | --- | --- |
| 1 | Pure Python on-device | Business logic, API clients, validation, utilities |
| 2 | Curated native packages | Selected mobile-ready packages compiled and tested by the project |
| 3 | Remote/edge backend | Heavy AI, data science, model inference, and large file processing |

## Important Rule

PyNative UI should not promise that every Python package runs on mobile. Heavy packages such as PyTorch, TensorFlow, and large Pandas workloads should normally use backend mode.
