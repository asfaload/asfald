# We run this customised server to server the tests' mirrors because
# windows does not support having ':' in a filename. This limitation
# prevents us to store the same paths as in the mirror under the tests
# directory (or the project would not be clonable or buildable under windows).
# Hence we replace the ':' in the tests mirror paths by _ and make
# a rewrite in this web server to still serve it with a ":" in the
# URL's path.
# The script takes the port to listen to as unique argument.
from http.server import HTTPServer, SimpleHTTPRequestHandler
import os
import sys

class RequestHandler(SimpleHTTPRequestHandler):
    def translate_path(self, path):
        # The server is started in the mirror's root
        root = os.getcwd()
        # Rewrite the path received to serve the file on disk
        new_path = path.replace(":","_",1)
        # Return full path to the file on disk
        return root+new_path

if __name__ == '__main__':
    # Listens on localhost and port passed as unique argument.
    web_server = HTTPServer(('127.0.0.1', int(sys.argv[1])), RequestHandler)
    try:
        web_server.serve_forever()
    except KeyboardInterrupt:
        pass

    web_server.server_close()
    print("Done.")
