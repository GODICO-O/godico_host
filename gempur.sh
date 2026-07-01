#!/bin/bash
HIJAU='\033[0;32m'; BIRU='\033[0;34m'; KUNING='\033[1;33m'; MERAH='\033[0;31m'; NORMAL='\033[0m'

echo -e "${BIRU}====================================================${NORMAL}"
echo -e "${HIJAU}   🗿 GODICO AUTOMATED PUSH, WATCH & INSTALLER 🗿   ${NORMAL}"
echo -e "${BIRU}====================================================${NORMAL}"

echo -e "\n${KUNING}[1/4] Mengirim kode ke Server GitHub (Tanpa Compile Lokal)...${NORMAL}"
git add .
git commit -m "Auto-deploy: Trigger GitHub Action Build"
git push origin main || { echo -e "${MERAH}Gagal Push!${NORMAL}"; exit 1; }

echo -e "\n${KUNING}[2/4] Mengawasi robot GitHub Actions (Build di Server)...${NORMAL}"
gh run watch --exit-status

echo -e "\n${KUNING}[3/4] Mengunduh Biner APK dari Awan...${NORMAL}"
rm -rf tmp_godico_apk && mkdir -p tmp_godico_apk
RUN_ID=$(gh run list --workflow="android.yml" --limit 1 --json databaseId -q '.[0].databaseId')
gh run download $RUN_ID -n GODICO-DevHub-APK --dir tmp_godico_apk

[ -f tmp_godico_apk/app-debug.apk ] || { echo -e "${MERAH}Gagal download APK!${NORMAL}"; exit 1; }

echo -e "\n${KUNING}[4/4] Mengirim instruksi eksekusi ke Android...${NORMAL}"
cp tmp_godico_apk/app-debug.apk /sdcard/Download/godico-devhub.apk
am start -a android.intent.action.VIEW \
    -d "file:///sdcard/Download/godico-devhub.apk" \
    -t "application/vnd.android.package-archive" --user 0 > /dev/null 2>&1

rm -rf tmp_godico_apk
echo -e "\n${HIJAU}✅ BERHASIL! Pelototi layar HP lu sekarang!${NORMAL}"
