import urllib.request
import shutil
import gzip
import re

SITEMAP_GZ_URL = "https://developer.mozilla.org/sitemaps/en-us/sitemap.xml.gz"

# Step 1: Pull down and extract site map into array of strings
URLS = []
with urllib.request.urlopen(SITEMAP_GZ_URL) as f:
    gz_data = f.read()
    data = gzip.decompress(gz_data)
    pattern = re.compile(r'/docs([^<]+)')
    
    for url in re.findall(pattern, f"{data}"):
        URLS.append(url)


# Step 2: write array of string to file

def url_is_useful(s):
    ignored_categories = [
            "MDN",
            "Glossary",
            "Games",
            "Mozilla",
            "Learn"
    ]

    for category in ignored_categories:
        if category in s:
            return False

    return True

with open('./data/mdn_urls.txt', 'w') as f:
    for line in URLS:
        if url_is_useful(line):
            f.write(f"{line}\n")
