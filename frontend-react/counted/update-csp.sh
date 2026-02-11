#!/bin/bash
# Script to update nginx CSP header after build

CSP_FILE="./dist/csp-header.txt"
NGINX_CSP_FILE="./csp-policy.conf"

if [ ! -f "$CSP_FILE" ]; then
    echo "❌ CSP header file not found: $CSP_FILE"
    echo "   Run 'npm run build' first"
    exit 1
fi

CSP_CONTENT=$(cat "$CSP_FILE")

cat > "$NGINX_CSP_FILE" <<EOF
# Auto-generated CSP policy - DO NOT EDIT MANUALLY
# Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
# To update: npm run build && ./update-csp.sh

add_header Content-Security-Policy "${CSP_CONTENT}" always;
EOF

echo "✅ CSP policy updated in $NGINX_CSP_FILE"
echo ""
echo "Next steps:"
echo "1. Copy csp-policy.conf to your nginx container"
echo "2. Reload nginx: docker-compose exec nginx nginx -s reload"
