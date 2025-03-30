import threading
import os
import subprocess

files_to_process = 0

def encrypt_files(files, password, callback):
    global files_to_process
    def runInThread(exitConvert, arg):
        proc = subprocess.Popen(arg, shell=True)
        proc.wait()
        exitConvert(password, callback)
    
    for file in files:
        exe_path = os.path.join(os.getcwd(),  "file_converter.exe")
        print(exe_path + " --convert " + file.path.replace("/","\\"))

        files_to_process += 1
        thread = threading.Thread(target=runInThread, args=(exitConvert, exe_path + " --convert \"" + file.path.replace("/","\\") + "\""))
        thread.start()


def exitConvert(password, callback):
    global files_to_process
    files_to_process -= 1
    print(files_to_process, "files remaining")
    if files_to_process == 0:
        print("Finished converting")

        def runInThread(exitPack, arg):
            proc = subprocess.Popen(arg, shell=True)
            proc.wait()
            exitPack(password, callback)

        exe_path = os.path.join(os.getcwd(),  "packer.exe")
        files = os.listdir(os.path.join(os.getcwd(), "output"))
        files = ["output/" +  file for file in files]
        files = ["\"" + file.replace("/","\\") + "\"" for file in files]

        print(exe_path + " --pack " + " ".join(files))
        thread = threading.Thread(target=runInThread, args=(exitPack, exe_path + " --pack " + " ".join(files)))
        thread.start()

def exitPack(password, callback):
    print("Finished packing")

    #copy and rename file packed.e to capsule.eternal
    packed_path = os.path.join(os.getcwd(), "packed.e")
    new_path = os.path.join(os.getcwd(), "capsule.eternal")
    if os.path.exists(new_path):
        os.remove(new_path)
    os.rename(packed_path, new_path)

    exitEncrypt(callback)

    """def runInThread(exitEncrypt, arg):
            proc = subprocess.Popen(arg, shell=True)
            proc.wait()
            exitEncrypt(callback)
    
    exe_path = os.path.join(os.getcwd(), "encrypter.exe")
    for file in os.listdir(os.path.join(os.getcwd(),  "output")):
        os.remove(os.path.join(os.getcwd(), "output", file))

    print(exe_path + " --encrypt " + password)
    thread = threading.Thread(target=runInThread, args=(exitEncrypt, exe_path + " --encrypt " + password))
    thread.start()"""

def exitEncrypt(callback):
    #os.remove(os.path.join(os.getcwd(), "packed.e"))
    print("Finished encrypting")
    callback()