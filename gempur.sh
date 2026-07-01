#!/bin/bash

# Warna untuk estetika terminal
HIJAU='\033[0;32m'; BIRU='\033[0;34m'; KUNING='\033[1;33m'; MERAH='\033[0;31m'; NORMAL='\033[0m'

echo -e "${BIRU}====================================================${NORMAL}"
echo -e "${HIJAU}   🗿 GODICO AUTOMATED PUSH, WATCH & INSTALLER 🗿   ${NORMAL}"
echo -e "${BIRU}====================================================${NORMAL}"

# 1. OTOMATIS GAMPUR PUSH KE GITHUB
echo -e "\n${KUNING}[1/4] Mengunci kode dan mengirim ke awan GitHub...${NORMAL}"
git add .
git commit -m "Auto-build: Sinkronisasi kode untuk CI/CD"
git push origin main

if [ $? -ne 0 ]; then
    echo -e "${MERAH}❌ Gagal melakukan push! Periksa koneksi atau repositori lu.${NORMAL}"
    exit 1
fi

# 2. OTOMATIS MEMANTAU BUILD (GH RUN WATCH)
echo -e "\n${KUNING}[2/4] Menghubungi robot GitHub Actions. Memulai pengawasan...${NORMAL}"
gh run watch

# 3. OTOMATIS DOWNLOAD ARTIFACT DARI RUN TERBARU
echo -e "\n${KUNING}[3/4] Build Sukses! Mengunduh biner APK dari awan...${NORMAL}"
rm -rf tmp_godico_apk && mkdir -p tmp_godico_apk

RUN_ID=$(gh run list --workflow="android.yml" --limit 1 --json databaseId -q '.[0].databaseId')
gh run download $RUN_ID -n GODICO-DevHub-APK --dir tmp_godico_apk

if [ ! -f tmp_godico_apk/app-debug.apk ]; then
    echo -e "${MERAH}❌ Gagal mengunduh atau mengekstrak APK!${NORMAL}"
    exit 1
fi

# 4. MEMICU POP-UP INSTALASI
echo -e "\n${KUNING}[4/4] Mengirim instruksi eksekusi ke Android...${NORMAL}"
cp tmp_godico_apk/app-debug.apk /sdcard/Download/godico-devhub.apk

am start -a android.intent.action.VIEW \
    -d "file:///sdcard/Download/godico-devhub.apk" \
    -t "application/vnd.android.package-archive" --user 0 > /dev/null 2>&1

rm -rf tmp_godico_apk

echo -e "\n${HIJAU}====================================================${NORMAL}"
echo -e "${HIJAU}✅ SCRIPT SELESAI! Pelototi layar HP lu sekarang!${NORMAL}"
echo -e "${BIRU}====================================================${NORMAL}"
