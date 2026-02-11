# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅ Yes    |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly.

### How to Report

1. **Do not** create a public issue
2. Send an email to: security@example.com (replace with your email)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### Response Timeline

- **Initial response**: Within 48 hours
- **Detailed assessment**: Within 7 days
- **Fix release**: As soon as possible, typically within 14 days

### What Happens Next

1. We will acknowledge receipt of your report
2. We will investigate and validate the vulnerability
3. We will develop a fix
4. We will coordinate a release date with you
5. We will credit you in the release notes (if desired)

## Security Features

KiloCode is designed with security as a top priority:

### 1. HTTPS-Only Communication

All API requests use HTTPS exclusively:

```rust
let response = client
    .post(format!("{}/chat/completions", endpoint))
    .header("Authorization", format!("Bearer {}", api_key))
    .json(&request)
    .send()
    .await?;
```

### 2. No Code Execution

KiloCode **never executes code**. AI responses are displayed as text only:

- No `eval()` or similar functions
- No shell command execution
- No file system operations beyond reading configuration
- No network requests except to configured API endpoints

### 3. Input Validation

All user inputs are validated before processing:

```rust
fn validate_input(input: &str) -> Result<(), String> {
    if input.len() > 100_000 {
        return Err("Input too large (max 100,000 characters)".to_string());
    }
    Ok(())
}
```

### 4. Output Sanitization

AI responses are sanitized to remove potentially harmful content:

```rust
fn sanitize_output(output: &str) -> String {
    output
        .replace("```bash", "```")
        .replace("```sh", "```")
        .replace("```shell", "```")
        .replace("```powershell", "```")
        .replace("```cmd", "```")
}
```

### 5. User-Controlled Credentials

- API keys are stored in user's Zed settings
- No hardcoded credentials in the extension
- Users provide their own API endpoint and key

### 6. Open Source

The entire codebase is open source for transparency and auditing.

## User Best Practices

### 1. Protect Your API Key

- Never share your API key
- Use environment-specific keys when possible
- Rotate keys regularly
- Revoke unused keys

### 2. Use HTTPS Endpoints

Only use API endpoints that support HTTPS:

```json
{
  "extensions": {
    "kilocode": {
      "api_endpoint": "https://your-api-endpoint.com/v1"
    }
  }
}
```

### 3. Review AI Responses

Always review AI-generated code before using it:
- Check for security vulnerabilities
- Verify logic correctness
- Test in a safe environment first

### 4. Keep Extension Updated

Enable automatic updates in Zed to receive security patches:

```json
{
  "extensions": {
    "auto_update": true
  }
}
```

### 5. Use Secure API Providers

Choose API providers that:
- Use HTTPS
- Have good security practices
- Provide rate limiting
- Offer audit logs

## Known Limitations

### 1. API Key Storage

API keys are stored in Zed's settings file, which is:
- Stored locally on your machine
- Encrypted on some platforms
- Accessible to anyone with file system access

**Recommendation**: Use API keys with limited scope and permissions.

### 2. Network Communication

The extension makes network requests to configured API endpoints:
- Requests are logged by the API provider
- Content may be processed by the API provider

**Recommendation**: Review your API provider's privacy policy.

### 3. Code Context

When using commands like `/kc-explain`, the selected code is sent to the API:
- Code may contain sensitive information
- Code is processed by the API provider

**Recommendation**: Be careful when sharing proprietary or sensitive code.

## Security Audits

This extension has not yet undergone a formal security audit. We welcome security researchers to review the code and report vulnerabilities.

## Dependency Security

We regularly update dependencies to address security vulnerabilities:

```bash
cargo update
cargo audit
```

### Current Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| zed_extension_api | 0.1.0 | Zed extension API |
| serde | 1.0 | Serialization |
| serde_json | 1.0 | JSON handling |
| reqwest | 0.12 | HTTP client |
| tokio | 1.0 | Async runtime |

## Compliance

This extension is designed to comply with:

- **OWASP Secure Coding Practices**
- **Rust Security Best Practices**
- **Zed Extension Security Guidelines**

## Contact

For security-related questions:
- Email: security@example.com (replace with your email)
- GitHub Security Advisories: https://github.com/YOUR_USERNAME/kilocode-zed/security/advisories

## License

This extension is licensed under the MIT License. See [LICENSE](LICENSE) for details.
