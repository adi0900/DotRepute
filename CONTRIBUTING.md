# Contributing to DotRepute

Thank you for your interest in contributing to DotRepute! We welcome contributions from the community to help build a robust contributor reputation system for the Polkadot ecosystem.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Workflow](#development-workflow)
4. [Contribution Guidelines](#contribution-guidelines)
5. [Pull Request Process](#pull-request-process)
6. [Coding Standards](#coding-standards)
7. [Testing Guidelines](#testing-guidelines)
8. [Documentation](#documentation)
9. [Community](#community)

---

## Code of Conduct

This project adheres to a Code of Conduct that all contributors are expected to follow. Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before contributing.

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors, regardless of:
- Experience level
- Gender identity and expression
- Sexual orientation
- Disability
- Personal appearance
- Body size
- Race
- Ethnicity
- Age
- Religion
- Nationality

---

## Getting Started

### Prerequisites

Before you begin, ensure you have:

- **Node.js** v18+ installed
- **Git** installed
- **GitHub account** configured
- Basic knowledge of TypeScript, React, and Polkadot ecosystem
- Read the [README.md](README.md) and [docs/architecture.md](docs/architecture.md)

### Initial Setup

1. **Fork the repository**
   ```bash
   # Click the "Fork" button on GitHub
   ```

2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/dotrepute.git
   cd dotrepute
   ```

3. **Add upstream remote**
   ```bash
   git remote add upstream https://github.com/adi0900/polkadot-25.git
   ```

4. **Install dependencies**
   ```bash
   # Frontend
   cd frontend
   npm install

   # Backend (if applicable)
   cd ../backend
   npm install

   # Indexer (if applicable)
   cd ../indexer
   npm install
   ```

5. **Set up environment variables**
   ```bash
   # Copy example env files
   cp frontend/.env.example frontend/.env.local
   cp backend/.env.example backend/.env
   ```

6. **Run the development server**
   ```bash
   # Frontend
   cd frontend
   npm run dev

   # Backend
   cd backend
   npm run dev
   ```

---

## Development Workflow

### 1. Create a Branch

Always create a new branch for your work:

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
# or
git checkout -b docs/documentation-update
```

**Branch naming conventions**:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions or modifications
- `chore/` - Maintenance tasks

### 2. Make Your Changes

- Write clean, readable code
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed
- Commit frequently with clear messages

### 3. Commit Your Changes

Use conventional commit messages:

```bash
git add .
git commit -m "feat: add governance score calculation"
# or
git commit -m "fix: resolve identity verification bug"
# or
git commit -m "docs: update API documentation"
```

**Commit message format**:
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style changes (formatting, etc.)
- `refactor` - Code refactoring
- `test` - Test additions or changes
- `chore` - Maintenance tasks
- `perf` - Performance improvements

**Example**:
```
feat(scoring): implement time decay for governance votes

- Add exponential decay function with 180-day half-life
- Apply decay to governance participation score
- Update tests to verify decay calculation

Closes #123
```

### 4. Keep Your Fork Updated

Regularly sync with the upstream repository:

```bash
git fetch upstream
git checkout main
git merge upstream/main
git push origin main
```

### 5. Rebase Your Branch

Before submitting a PR, rebase on the latest main:

```bash
git checkout your-branch-name
git rebase main
```

Resolve any conflicts, then:

```bash
git push origin your-branch-name --force-with-lease
```

---

## Contribution Guidelines

### What We're Looking For

We welcome contributions in the following areas:

#### High Priority
- 🐛 Bug fixes
- 📊 Scoring algorithm improvements
- 🎨 UI/UX enhancements
- 📚 Documentation improvements
- ✅ Test coverage expansion

#### Medium Priority
- ⚡ Performance optimizations
- 🔒 Security enhancements
- 🌐 Internationalization (i18n)
- ♿ Accessibility improvements

#### Low Priority
- 🎉 New features (discuss first in an issue)
- 🔧 Developer tooling
- 📦 Dependency updates

### Before Starting Work

1. **Check existing issues** - Someone may already be working on it
2. **Create or comment on an issue** - Discuss your approach
3. **Wait for approval** - Especially for major features
4. **Claim the issue** - Comment that you're working on it

### Issue Guidelines

When creating an issue:

**Bug Reports**:
```markdown
## Bug Description
[Clear description of the bug]

## Steps to Reproduce
1. Go to '...'
2. Click on '...'
3. See error

## Expected Behavior
[What should happen]

## Actual Behavior
[What actually happens]

## Screenshots
[If applicable]

## Environment
- OS: [e.g., macOS 12.0]
- Browser: [e.g., Chrome 98]
- Node version: [e.g., 18.12.0]
```

**Feature Requests**:
```markdown
## Feature Description
[Clear description of the feature]

## Problem It Solves
[What problem does this address?]

## Proposed Solution
[How would you implement it?]

## Alternatives Considered
[Other approaches you've thought about]

## Additional Context
[Any other relevant information]
```

---

## Pull Request Process

### Before Submitting

- [ ] Code follows the project's style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All tests pass locally
- [ ] No console errors or warnings
- [ ] Commit messages follow conventions

### Submitting a PR

1. **Push your branch**
   ```bash
   git push origin your-branch-name
   ```

2. **Create the PR**
   - Go to your fork on GitHub
   - Click "New Pull Request"
   - Select your branch
   - Fill out the PR template

3. **PR Template**
   ```markdown
   ## Description
   [Brief description of changes]

   ## Type of Change
   - [ ] Bug fix (non-breaking change)
   - [ ] New feature (non-breaking change)
   - [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
   - [ ] Documentation update

   ## Related Issues
   Closes #[issue number]

   ## Changes Made
   - [Change 1]
   - [Change 2]
   - [Change 3]

   ## How Has This Been Tested?
   - [ ] Unit tests
   - [ ] Integration tests
   - [ ] Manual testing

   ## Screenshots (if applicable)
   [Add screenshots]

   ## Checklist
   - [ ] My code follows the style guidelines
   - [ ] I have performed a self-review
   - [ ] I have commented my code, particularly in hard-to-understand areas
   - [ ] I have made corresponding changes to the documentation
   - [ ] My changes generate no new warnings
   - [ ] I have added tests that prove my fix is effective or that my feature works
   - [ ] New and existing unit tests pass locally
   - [ ] Any dependent changes have been merged and published
   ```

### PR Review Process

1. **Automated Checks**
   - CI/CD pipeline runs tests
   - Code quality checks (ESLint, Prettier)
   - Type checking (TypeScript)
   - Build verification

2. **Code Review**
   - At least one maintainer will review
   - Address feedback promptly
   - Push updates to the same branch

3. **Approval & Merge**
   - Once approved, a maintainer will merge
   - PR will be squashed or rebased as appropriate

### After Merge

- Delete your branch (locally and on GitHub)
- Update your local main branch
- Celebrate! 🎉

---

## Coding Standards

### TypeScript

```typescript
// Use TypeScript strict mode
// Always define types explicitly

// ✅ Good
interface UserScore {
  address: string;
  score: number;
  breakdown: ScoreBreakdown;
}

function calculateScore(address: string): UserScore {
  // Implementation
}

// ❌ Bad
function calculateScore(address: any): any {
  // Implementation
}
```

### React Components

```typescript
// Use functional components with TypeScript
// Props should be explicitly typed

// ✅ Good
interface ScoreCardProps {
  score: number;
  label: string;
  variant?: 'primary' | 'secondary';
}

export const ScoreCard: React.FC<ScoreCardProps> = ({
  score,
  label,
  variant = 'primary'
}) => {
  return (
    <div className={`score-card ${variant}`}>
      <h3>{label}</h3>
      <p>{score}</p>
    </div>
  );
};

// ❌ Bad
export const ScoreCard = (props: any) => {
  return <div>{props.score}</div>;
};
```

### Naming Conventions

```typescript
// Components: PascalCase
export const ReputationDashboard = () => {};

// Functions: camelCase
function calculateReputationScore() {}

// Constants: SCREAMING_SNAKE_CASE
const MAX_SCORE = 100;
const API_ENDPOINT = 'https://api.dotrepute.io';

// Types/Interfaces: PascalCase
interface ReputationScore {}
type ScoreBreakdown = {};

// Files: kebab-case
// reputation-dashboard.tsx
// score-calculator.ts
```

### Code Organization

```
component/
├── reputation-card.tsx        # Component implementation
├── reputation-card.test.tsx   # Tests
├── reputation-card.styles.ts  # Styles (if not using Tailwind)
└── index.ts                   # Barrel export
```

### Comments

```typescript
// Use JSDoc for functions and complex logic

/**
 * Calculates the reputation score for a given address
 * @param address - The Polkadot address to score
 * @param options - Optional scoring parameters
 * @returns The calculated reputation score (0-100)
 * @throws {InvalidAddressError} If address format is invalid
 */
export async function calculateReputationScore(
  address: string,
  options?: ScoringOptions
): Promise<number> {
  // Validate address format
  if (!isValidAddress(address)) {
    throw new InvalidAddressError(address);
  }

  // Fetch data from multiple sources
  const [identity, governance, staking] = await Promise.all([
    fetchIdentityData(address),
    fetchGovernanceData(address),
    fetchStakingData(address),
  ]);

  // Calculate weighted score
  return computeWeightedScore({
    identity,
    governance,
    staking,
  });
}
```

---

## Testing Guidelines

### Test Structure

```typescript
import { describe, it, expect } from '@jest/globals';
import { calculateReputationScore } from './reputation-calculator';

describe('calculateReputationScore', () => {
  it('should return 0 for addresses with no activity', async () => {
    const score = await calculateReputationScore('5GrwvaEF...');
    expect(score).toBe(0);
  });

  it('should apply correct weights to each component', async () => {
    const score = await calculateReputationScore('5GrwvaEF...', {
      weights: {
        identity: 0.3,
        governance: 0.3,
        staking: 0.2,
        activity: 0.2,
      },
    });
    expect(score).toBeGreaterThan(0);
    expect(score).toBeLessThanOrEqual(100);
  });

  it('should throw error for invalid address format', async () => {
    await expect(
      calculateReputationScore('invalid-address')
    ).rejects.toThrow(InvalidAddressError);
  });
});
```

### Test Coverage

Aim for:
- **Unit tests**: 80%+ coverage
- **Integration tests**: Critical user flows
- **E2E tests**: Main user journeys

### Running Tests

```bash
# Run all tests
npm test

# Run with coverage
npm run test:coverage

# Run specific test file
npm test -- reputation-calculator.test.ts

# Run in watch mode
npm test -- --watch
```

---

## Documentation

### Code Documentation

- Add JSDoc comments for public APIs
- Document complex algorithms
- Include usage examples
- Keep comments up-to-date

### README Updates

Update README.md when:
- Adding new features
- Changing setup instructions
- Modifying dependencies
- Updating configuration

### Architecture Documentation

Update `docs/architecture.md` when:
- Adding new components
- Changing data flow
- Modifying integrations
- Updating tech stack

### API Documentation

Update `docs/api-spec.md` when:
- Adding new endpoints
- Changing request/response formats
- Modifying authentication

---

## Community

### Getting Help

- 💬 **GitHub Discussions**: For questions and general discussion
- 🐛 **GitHub Issues**: For bug reports and feature requests
- 🔧 **Discord**: [Join our server](#) (link TBD)
- 📧 **Email**: dotrepute@example.com

### Communication Guidelines

- Be respectful and professional
- Stay on topic
- Search before asking
- Provide context and details
- Follow up on your issues/PRs

### Recognition

We value all contributions! Contributors will be:
- Listed in CONTRIBUTORS.md
- Credited in release notes
- Mentioned in project updates

---

## Project Maintainers

- **Aditya** (@adi0900) - Product Designer & Product Manager
- **Steven Muanigo** (@stevenmuanigo) - Backend & Infrastructure Developer

### Maintainer Responsibilities

Maintainers will:
- Review pull requests
- Triage issues
- Guide project direction
- Ensure code quality
- Maintain documentation
- Foster community

---

## Development Tips

### Useful Commands

```bash
# Type checking
npm run type-check

# Linting
npm run lint
npm run lint:fix

# Formatting
npm run format

# Build
npm run build

# Clean build artifacts
npm run clean
```

### Debugging

```typescript
// Use debug logging
import debug from 'debug';
const log = debug('dotrepute:scoring');

log('Calculating score for address: %s', address);
```

### Performance Profiling

```typescript
console.time('scoreCalculation');
const score = await calculateReputationScore(address);
console.timeEnd('scoreCalculation');
```

---

## License

By contributing to DotRepute, you agree that your contributions will be licensed under the MIT License.

---

## Questions?

If you have any questions about contributing, please:
1. Check existing documentation
2. Search GitHub issues
3. Ask in GitHub Discussions
4. Contact the maintainers

**Thank you for contributing to DotRepute! Together, we're building a more transparent and trustworthy Polkadot ecosystem.** 🚀
