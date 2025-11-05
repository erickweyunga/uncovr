# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Uncovr framework
- Type-safe API endpoint system with compile-time validation
- Built-in OpenAPI/Scalar documentation generation
- Comprehensive configuration system with environment presets (Development, Production, Staging)
- CORS middleware with configurable origins and headers
- Structured logging with tracing integration
- Request/response logging with method, status, latency, and user agent
- Configurable log levels (Debug, Info, Warn, Error) and formats (Pretty, JSON)
- Builder patterns for all configuration types
- Support for JSON request/response serialization
- Automatic schema generation with schemars
- API example demonstrating minimal API setup

### Features
- **Configuration**: Environment-based presets with full customization support
- **Logging**: Structured logging with configurable levels and formats
- **CORS**: Restrictive by default, configurable per environment
- **Documentation**: Auto-generated OpenAPI specs with Scalar UI
- **Type Safety**: Compile-time validation of API endpoints

## [0.1.0] - 2025-11-04

### Initial Release
- First working version of Uncovr framework
