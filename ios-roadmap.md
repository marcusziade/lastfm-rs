# Last.fm iOS App Roadmap

A comprehensive roadmap for building a feature-complete iOS app using UIKit that mirrors all functionality of the lastfm-rs CLI application with beautifully designed user interfaces, smooth paging behavior, and offline capabilities using GRDB.

## Overview

This iOS app will provide a native, performant interface to the Last.fm API with all features from the CLI tool, including:
- Complete API coverage (30+ endpoints)
- Authentication and personal data access
- Offline caching with GRDB
- Multiple view formats (table, pretty, JSON, compact)
- Search functionality with pagination
- Time period filtering
- Rich error handling

## Technology Stack

- **UI Framework**: UIKit (programmatic)
- **Language**: Swift 5.9+
- **Minimum iOS**: 15.0
- **Database**: GRDB for offline caching
- **Networking**: URLSession with async/await
- **Architecture**: MVVM-C (Model-View-ViewModel-Coordinator)
- **Dependency Injection**: Manual DI container
- **Testing**: XCTest for unit and integration tests

## Phase 1: Core Infrastructure & Architecture

### 1.1 Project Setup & Core Architecture
- [ ] Create Xcode project with UIKit app template
- [ ] Set up SPM dependencies (GRDB)
- [ ] Configure project structure (Features, Core, Resources)
- [ ] Implement dependency injection container
- [ ] Create base coordinator protocol and implementation
- [ ] Set up navigation controller architecture
- [ ] Configure build settings and environments (dev, staging, prod)

### 1.2 Networking Layer
- [ ] Create `APIClient` protocol with async/await support
- [ ] Implement `LastFMAPIClient` with URLSession
- [ ] Define all API endpoint enums matching CLI commands
- [ ] Create request builder with parameter validation
- [ ] Implement response decoders for all data models
- [ ] Add request signing for authenticated endpoints
- [ ] Create rate limiting handler
- [ ] Implement network reachability monitoring

### 1.3 Data Models & Domain Layer
- [ ] Define all data models (Artist, Album, Track, User, etc.)
- [ ] Create domain protocols for data repositories
- [ ] Implement model serialization/deserialization
- [ ] Create value objects for time periods, countries, tags
- [ ] Define error types matching CLI error handling
- [ ] Create response metadata models

### 1.4 Database Layer (GRDB)
- [ ] Set up GRDB database manager
- [ ] Create database migrations system
- [ ] Define database schemas for all models
- [ ] Implement CRUD operations for each entity
- [ ] Create cache expiration logic
- [ ] Implement database observers for reactive updates
- [ ] Add database queue management
- [ ] Create offline-first data synchronization

### 1.5 Configuration Management
- [ ] Create `ConfigurationManager` for app settings
- [ ] Implement UserDefaults wrapper with property wrappers
- [ ] Define configuration models (API settings, UI preferences)
- [ ] Create settings migration system
- [ ] Add configuration validation
- [ ] Implement secure storage for sensitive data

## Phase 2: Authentication & User Management

### 2.1 Authentication Flow
- [ ] Create authentication coordinator
- [ ] Implement OAuth-style login view controller
- [ ] Build web view for Last.fm authorization
- [ ] Create token exchange mechanism
- [ ] Implement secure keychain storage for session
- [ ] Add biometric authentication support
- [ ] Create logout and session clearing
- [ ] Implement auto-refresh for expired sessions

### 2.2 User Profile Management
- [ ] Create user profile view controller
- [ ] Implement profile data repository
- [ ] Add user statistics display
- [ ] Create listening history visualization
- [ ] Implement profile caching in GRDB
- [ ] Add pull-to-refresh functionality
- [ ] Create profile editing capabilities

### 2.3 Session Management
- [ ] Implement session state manager
- [ ] Create session persistence
- [ ] Add session validation on app launch
- [ ] Implement guest mode support
- [ ] Create session timeout handling
- [ ] Add multi-account support structure

## Phase 4: Feature Implementation

### 4.1 Artist Features
- [ ] Create `ArtistListViewController` with search
- [ ] Implement `ArtistDetailViewController`
- [ ] Add artist info display with rich media
- [ ] Create similar artists carousel view
- [ ] Implement top albums collection view
- [ ] Add top tracks table view
- [ ] Create artist biography expandable view
- [ ] Implement artist image gallery

### 4.2 Album Features
- [ ] Create `AlbumSearchViewController`
- [ ] Implement `AlbumDetailViewController`
- [ ] Add album artwork display with zoom
- [ ] Create track listing with duration
- [ ] Implement album info and metadata
- [ ] Add album statistics view
- [ ] Create share functionality

### 4.3 Track Features
- [ ] Create `TrackSearchViewController`
- [ ] Implement `TrackDetailViewController`
- [ ] Add track info with artist/album links
- [ ] Create similar tracks recommendation view
- [ ] Implement playback integration hooks
- [ ] Add track statistics display
- [ ] Create love/unlove functionality

### 4.4 Chart Features
- [ ] Create `ChartsTabViewController`
- [ ] Implement top artists chart with pagination
- [ ] Add top tracks chart with preview
- [ ] Create top tags cloud visualization
- [ ] Implement time period selector
- [ ] Add chart refresh functionality
- [ ] Create chart sharing capabilities

### 4.5 Geographic Features
- [ ] Create `GeoViewController` with map
- [ ] Implement country selector
- [ ] Add top artists by country list
- [ ] Create top tracks by country list
- [ ] Implement geographic visualization
- [ ] Add country comparison feature

### 4.6 Tag Features
- [ ] Create `TagBrowserViewController`
- [ ] Implement tag search functionality
- [ ] Add tag info display
- [ ] Create tagged artists collection
- [ ] Implement tagged albums grid
- [ ] Add tagged tracks list
- [ ] Create tag following system

### 4.7 Library Features
- [ ] Create `LibraryViewController`
- [ ] Implement artists grid view
- [ ] Add sorting and filtering options
- [ ] Create batch operations support
- [ ] Implement library statistics
- [ ] Add export functionality

### 4.8 Personal Data Features (Authenticated)
- [ ] Create `MyDataTabViewController`
- [ ] Implement recent tracks with timeline
- [ ] Add top artists with time period filter
- [ ] Create top tracks visualization
- [ ] Implement top albums grid
- [ ] Add listening statistics dashboard
- [ ] Create data export functionality

### 4.9 Search Implementation
- [ ] Create unified `SearchViewController`
- [ ] Implement search scope selector
- [ ] Add search history with suggestions
- [ ] Create real-time search with debouncing
- [ ] Implement search results pagination
- [ ] Add search filters
- [ ] Create recent searches persistence

### 4.10 Output Format Views
- [ ] Create `TableFormatViewController` for tabular data
- [ ] Implement `PrettyFormatViewController` with rich UI
- [ ] Add `JSONViewerViewController` with syntax highlighting
- [ ] Create `CompactListViewController` for dense data
- [ ] Implement format switcher in navigation bar
- [ ] Add format preferences persistence

## Phase 5: Polish & Performance

### 5.1 UI/UX Enhancement
- [ ] Implement custom transitions and animations
- [ ] Add haptic feedback for interactions
- [ ] Create loading states and skeletons
- [ ] Implement empty states with illustrations
- [ ] Add pull-to-refresh on all scrollable views
- [ ] Create smooth infinite scrolling
- [ ] Implement keyboard avoidance
- [ ] Add accessibility support (VoiceOver, Dynamic Type)

### 5.2 Performance Optimization
- [ ] Implement image caching with progressive loading
- [ ] Add list view cell pre-fetching
- [ ] Create background data synchronization
- [ ] Implement incremental search indexing
- [ ] Add memory usage monitoring
- [ ] Create performance profiling hooks
- [ ] Implement lazy loading for heavy views

### 5.3 Error Handling & Recovery
- [ ] Create comprehensive error UI components
- [ ] Implement retry mechanisms with backoff
- [ ] Add offline mode indicators
- [ ] Create error reporting system
- [ ] Implement graceful degradation
- [ ] Add user-friendly error messages
- [ ] Create error recovery suggestions

### 5.4 App Features
- [ ] Implement deep linking support
- [ ] Add widget extensions
- [ ] Create Siri shortcuts integration
- [ ] Implement universal links
- [ ] Add share extensions
- [ ] Create notification support
- [ ] Implement background refresh

### 5.5 Settings & Preferences
- [ ] Create `SettingsViewController`
- [ ] Implement appearance settings (theme, colors)
- [ ] Add data management (cache, downloads)
- [ ] Create privacy settings
- [ ] Implement notification preferences
- [ ] Add advanced options (timeout, retries)
- [ ] Create about section with credits

### 5.6 Testing & Quality
- [ ] Write unit tests for all ViewModels
- [ ] Create integration tests for repositories
- [ ] Implement UI tests for critical flows
- [ ] Add performance tests
- [ ] Create mock data generators
- [ ] Implement analytics tracking
- [ ] Add crash reporting

### 5.7 Final Polish
- [ ] Create app icon and launch screen
- [ ] Implement onboarding flow
- [ ] Add tutorial tooltips
- [ ] Create release notes system
- [ ] Implement feature flags
- [ ] Add A/B testing framework
- [ ] Create feedback collection

## Deliverables per Phase

### Phase 1 Deliverables
- Fully architected app foundation
- Complete networking layer
- Database schema and operations
- All data models defined
- Configuration system

### Phase 2 Deliverables
- Working authentication flow
- User profile management
- Secure session handling
- Guest mode support

### Phase 4 Deliverables
- All CLI features implemented
- Rich native UI for each feature
- Offline support for all data
- Search functionality
- Multiple view formats

### Phase 5 Deliverables
- Polished, performant app
- Comprehensive error handling
- Accessibility support
- App Store ready build
- Complete test coverage

## Success Metrics

- [ ] 100% feature parity with CLI tool
- [ ] < 2 second cold launch time
- [ ] Offline access to cached data
- [ ] 60 fps scrolling performance
- [ ] < 100 MB app size
- [ ] 4.5+ star App Store rating target
- [ ] < 0.1% crash rate
- [ ] 90%+ code coverage

## Architecture Principles

1. **Offline-First**: All data cached in GRDB, sync when online
2. **Reactive**: Use Combine/delegates for data flow
3. **Modular**: Feature-based modules with clear boundaries
4. **Testable**: Dependency injection, protocol-oriented
5. **Performant**: Lazy loading, efficient memory usage
6. **Accessible**: Full VoiceOver and Dynamic Type support
7. **Secure**: Keychain storage, certificate pinning

## Key Technical Decisions

- **No Storyboards**: Programmatic UI for better collaboration
- **GRDB over Core Data**: Simpler API, better performance
- **Manual DI**: Avoid heavy DI frameworks
- **UIKit over SwiftUI**: Better performance, more control
- **Async/Await**: Modern concurrency over callbacks
- **MVVM-C**: Clear separation of concerns

## Risk Mitigation

- **API Changes**: Version checking, graceful fallbacks
- **Rate Limiting**: Client-side throttling, queue management
- **Large Data Sets**: Pagination, virtualized lists
- **Network Issues**: Offline mode, retry strategies
- **Memory Pressure**: Image downsampling, cache limits

This roadmap provides a complete path from empty project to a polished, App Store-ready iOS application that exceeds the functionality of the CLI tool while providing a beautiful, native iOS experience.
