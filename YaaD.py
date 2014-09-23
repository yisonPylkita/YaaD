#!/usr/bin/env python2
# -*- coding: utf-8 -*-

import urllib
from HTMLParser import HTMLParser
from termcolor import cprint
import os
import sys

class MyHTMLParser(HTMLParser):
    links = []
    def handle_starttag(self, tag, attrs):
        if tag == "a":
            if attrs[0][1].find("/infusions/video/video.php") != -1 or attrs[0][1].find("/viewpage.php") != -1:
                self.links.append(attrs[0][1])

connection = urllib.urlopen(sys.argv[1])
page = connection.read()
connection.close()
parser = MyHTMLParser()
parser.feed(page)
for x in parser.links:
    cprint(x, "green")
for x in parser.links:
    os.system("youtube-dl %s" % "http://www.anime-odcinki.pl"+x)
