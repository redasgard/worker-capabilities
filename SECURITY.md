# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take security bugs seriously. We appreciate your efforts to responsibly disclose your findings, and will make every effort to acknowledge your contributions.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to:

**security@redasgard.com**

### What to Include

When reporting a security vulnerability, please include:

1. **Description**: A clear description of the vulnerability
2. **Steps to Reproduce**: Detailed steps to reproduce the issue
3. **Impact**: Description of the potential impact
4. **Environment**: OS, Rust version, worker configuration, and any other relevant details
5. **Proof of Concept**: If possible, include a minimal code example that demonstrates the issue

### What to Expect

- **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
- **Initial Assessment**: We will provide an initial assessment within 5 business days
- **Regular Updates**: We will keep you informed of our progress
- **Resolution**: We will work with you to resolve the issue and coordinate disclosure

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 5 business days
- **Resolution**: Within 30 days (depending on complexity)

## Security Considerations

### Worker Capabilities Specific Concerns

When reporting vulnerabilities, please consider:

1. **Capability Escalation**: Unauthorized capability access
2. **Worker Impersonation**: Impersonation of legitimate workers
3. **Workflow Manipulation**: Malicious workflow execution
4. **Resource Exhaustion**: DoS through resource consumption
5. **Data Exfiltration**: Unauthorized data access
6. **Memory Safety**: Unsafe memory operations or buffer overflows

### Attack Vectors

Common attack vectors to test:

- **Capability Escalation**: Unauthorized access to capabilities
- **Worker Impersonation**: Impersonation of legitimate workers
- **Workflow Manipulation**: Malicious workflow execution
- **Resource Exhaustion**: DoS through resource consumption
- **Data Exfiltration**: Unauthorized data access
- **Privilege Escalation**: Unauthorized privilege escalation
- **Network Attacks**: Man-in-the-middle, DNS spoofing

## Security Best Practices

### For Users

1. **Validate Workers**: Only trust verified workers
2. **Implement Capability Controls**: Use proper capability controls
3. **Workflow Validation**: Validate workflows before execution
4. **Keep the library updated** to the latest version
5. **Monitor for security advisories**
6. **Implement proper access controls**

### For Developers

1. **Test with malicious workers** regularly
2. **Implement defense in depth**
3. **Use the library correctly** according to documentation
4. **Consider additional validation** for critical applications
5. **Monitor security updates**
6. **Implement proper worker validation**

## Security Features

### Built-in Protections

- **Capability Controls**: Capability-based security
- **Worker Validation**: Built-in worker validation
- **Workflow Security**: Secure workflow execution
- **Memory Safety**: Rust's memory safety guarantees
- **Type Safety**: Compile-time type safety
- **Configurable Security**: Adjustable security settings

### Additional Recommendations

- **Worker Authentication**: Implement worker authentication
- **Capability Controls**: Implement strict capability controls
- **Workflow Validation**: Validate workflows before execution
- **Access Controls**: Implement proper access controls
- **Logging**: Log security events for monitoring
- **Regular Updates**: Keep dependencies and the library updated

## Security Updates

### How We Handle Security Issues

1. **Assessment**: We assess the severity and impact
2. **Fix Development**: We develop a fix in private
3. **Testing**: We thoroughly test the fix
4. **Release**: We release the fix with a security advisory
5. **Disclosure**: We coordinate disclosure with reporters

### Security Advisories

Security advisories are published on:

- **GitHub Security Advisories**: https://github.com/redasgard/worker-capabilities/security/advisories
- **Crates.io**: Security notices in release notes
- **Email**: Subscribers to security@redasgard.com

## Responsible Disclosure

We follow responsible disclosure practices:

1. **Private Reporting**: Report vulnerabilities privately first
2. **Coordinated Disclosure**: We coordinate disclosure timing
3. **Credit**: We give credit to security researchers
4. **No Legal Action**: We won't take legal action against good faith research

## Security Research

### Guidelines for Security Researchers

- **Test Responsibly**: Don't test on production systems
- **Respect Privacy**: Don't access or modify data
- **Report Promptly**: Report findings as soon as possible
- **Follow Guidelines**: Follow this security policy

### Scope

**In Scope:**
- Capability escalation vulnerabilities
- Worker impersonation attacks
- Workflow manipulation attacks
- Resource exhaustion attacks
- Memory safety issues
- Performance DoS attacks

**Out of Scope:**
- Social engineering attacks
- Physical security issues
- Issues in dependencies (report to their maintainers)
- Issues in applications using this library
- Issues in workers themselves

## Contact

For security-related questions or to report vulnerabilities:

- **Email**: security@redasgard.com
- **PGP Key**: Available upon request
- **Response Time**: Within 48 hours

## Acknowledgments

We thank the security researchers who help keep our software secure. Security researchers who follow responsible disclosure practices will be acknowledged in our security advisories.

## Legal

By reporting a security vulnerability, you agree to:

1. **Not disclose** the vulnerability publicly until we've had a chance to address it
2. **Not access or modify** data that doesn't belong to you
3. **Not disrupt** our services or systems
4. **Act in good faith** to avoid privacy violations, destruction of data, and interruption or degradation of our services

Thank you for helping keep Worker Capabilities and our users safe! ⚡🛡️