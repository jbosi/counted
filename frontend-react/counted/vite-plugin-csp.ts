import { Plugin } from 'vite';
import crypto from 'crypto';
import fs from 'fs';
import path from 'path';

interface CSPPluginOptions {
	outputPath?: string;
}

export function viteCspPlugin(options: CSPPluginOptions = {}): Plugin {
	const hashes: Set<string> = new Set();

	return {
		name: 'vite-csp-plugin',
		apply: 'build',

		transformIndexHtml: {
			order: 'post',
			handler(html) {
				// Find all inline scripts and styles
				const scriptRegex = /<script(?![^>]*\ssrc=)[^>]*>([\s\S]*?)<\/script>/gi;
				const styleRegex = /<style[^>]*>([\s\S]*?)<\/style>/gi;

				let match;

				// Hash inline scripts
				while ((match = scriptRegex.exec(html)) !== null) {
					const content = match[1].trim();
					if (content) {
						const hash = crypto
							.createHash('sha256')
							.update(content)
							.digest('base64');
						hashes.add(`'sha256-${hash}'`);
					}
				}

				// Hash inline styles
				while ((match = styleRegex.exec(html)) !== null) {
					const content = match[1].trim();
					if (content) {
						const hash = crypto
							.createHash('sha256')
							.update(content)
							.digest('base64');
						hashes.add(`'sha256-${hash}'`);
					}
				}

				return html;
			},
		},

		closeBundle() {
			// Generate CSP header
			const scriptHashes = Array.from(hashes).join(' ');

			const cspHeader = [
				"default-src 'self'",
				`script-src 'self' ${scriptHashes}`,
				"style-src 'self' https://fonts.googleapis.com",
				"img-src 'self' data: https:",
				"font-src 'self' data: https://fonts.gstatic.com",
				"connect-src 'self'",
				"frame-src 'none'",
				"object-src 'none'",
				"base-uri 'self'",
				"form-action 'self'",
				'upgrade-insecure-requests',
				'report-uri /csp-report',
			].join('; ');

			// Output to file for nginx to include
			const outputPath =
				options.outputPath || path.join(process.cwd(), 'dist', 'csp-header.txt');
			fs.writeFileSync(outputPath, cspHeader, 'utf-8');

			console.log('\n✅ CSP header generated with hashes:');
			console.log(`   ${scriptHashes}`);
			console.log(`   Saved to: ${outputPath}\n`);
		},
	};
}
