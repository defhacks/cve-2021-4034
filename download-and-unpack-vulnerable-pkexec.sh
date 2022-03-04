#!/bin/sh

DEB_URL="https://launchpad.net/~ubuntu-security/+archive/ubuntu/ppa/+build/21593865/+files/policykit-1_0.105-26ubuntu1.1_amd64.deb"

mkdir -p playground
test -f ./playground/pkexec && echo "found pkexec" && exit 0

set -e

TEMP_DEB_DIR=$(mktemp -d)

wget -O ${TEMP_DEB_DIR}/vulnerable-polkit.deb ${DEB_URL}
pushd ${TEMP_DEB_DIR}
ar x vulnerable-polkit.deb data.tar.xz
tar -xf data.tar.xz ./usr/bin/pkexec
popd
cp ${TEMP_DEB_DIR}/usr/bin/pkexec playground/

echo '[*] successfully copied vulnerable pkexec to playground/pkexec'
echo '[*] Note: if you want to test privilege escalation, please chown/chmod accordingly'
echo '[*] e.g. chown playground/pkexec root:<your_user> && chmod 4750 playground/pkexec'
echo '[*] Cleaning up temporary download dir'

if [ "$(echo $TEMP_DEB_DIR | cut -c1-5)" = "/tmp/" ] ; then
  rm -rf "${TEMP_DEB_DIR}"
fi
