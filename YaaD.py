#!/usr/bin/env python2
# -*- coding: utf-8 -*-

# HTTP connection
import urllib
# page parsing (links to videos searching)
from HTMLParser import HTMLParser
# terminal output with some colors
from termcolor import cprint
# run external application
import os
import sys
# 
import re

tsc = []

class MyHTMLParser(HTMLParser):
    links = []
    def handle_starttag(self, tag, attrs):
        for attr in attrs:
            tsc.append(attr[1])

class MyHTMLParserNext(HTMLParser):
    links = []
    def handle_starttag(self, tag, attrs):
        if tag == "embed":
            for x in attrs:
                tsc.append(x[1])
url_link = sys.argv[1]
connection = urllib.urlopen(url_link)
page = connection.read()
connection.close()
parser = MyHTMLParser()
parser.feed(page)
d = []
for x in tsc:
   if type(x) == str:
       d.append(x)
raw = []
p = re.compile(r"^http://www.anime-shinden.info/.([0-9]|(-))+([a-z]|([0-9])|(-))+[0-9]+.html")
for x in d:
    match = p.match(x)
    if match:
        raw.append(x)
    else:
        continue
links = []
for x in raw:
    match = re.search(sys.argv[2], x)
    if match:
        links.append(x)
    else:
        continue
for x in links:
    cprint(x, "blue")
raw = []
tsc = []
pages = []

# Now is the time to load root childs
for x in links:
    tsc = []
    connection = urllib.urlopen(x)
    page = connection.read()
    connection.close()
    parser = MyHTMLParser()
    parser.feed(page)
    for x in tsc:
        match = re.search("http://www.anime-shinden.info/external-player", x)
        if match:
            raw.append(x)
            cprint(x, "red")
            break
        else:
            continue
    connection = urllib.urlopen(raw[0])
    page = connection.read()
    connection.close()
    tsc = []
    parser = MyHTMLParserNext()
    parser.feed(page)
    raw = []
    for x in tsc:
        match = x.find("http://anime-shinden.info/player/hd")
        if match != -1:
            match_end = x[match:].find("&")
            raw.append(x[match:match+match_end])
            cprint(x[match:match+match_end], "green")
        else:
            continue
        for x in raw:
            os.system("youtube-dl %s" % x)
        raw = []
