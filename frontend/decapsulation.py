import threading
import os
import subprocess

files_to_process = 0

def decrypt_file(password, callback):

    #rename capsule.etneral to packed.e
    packed_path = os.path.join(os.getcwd(), "capsule.eternal")
    new_path = os.path.join(os.getcwd(), "packed.e")
    if os.path.exists(new_path):
        os.remove(new_path)
    os.rename(packed_path, new_path)

    exitDecrypt(callback)

    # def runInThread(exitDecrypt, arg):
    #         proc = subprocess.Popen(arg, shell=True)
    #         proc.wait()
    #         exitDecrypt(callback)
    
    # exe_path = os.path.join(os.getcwd(), "encrypter.exe")

    # print(exe_path + " --decrypt " + password)
    # thread = threading.Thread(target=runInThread, args=(exitDecrypt, exe_path + " --decrypt " + password))
    # thread.start()


def exitDecrypt(callback):
    print("Finished decrypting")

    def runInThread(exitUnpack, arg):
        proc = subprocess.Popen(arg, shell=True)
        proc.wait()
        exitUnpack(callback)

    exe_path = os.path.join(os.getcwd(),  "packer.exe")
    
    thread = threading.Thread(target=runInThread, args=(exitUnpack, exe_path + " --unpack "))
    thread.start()

def exitUnpack(callback):
    global files_to_process
    print("Finished unpacking")

    def runInThread(exitDeconvert, arg):
            proc = subprocess.Popen(arg, shell=True)
            proc.wait()
            exitDeconvert(callback)
    
    #os.remove(os.path.join(os.getcwd(), "packed.e"))

    exe_path = os.path.join(os.getcwd(), "file_converter.exe")

    files = os.listdir(os.path.join(os.getcwd(), "output"))
    files = [os.path.join(os.getcwd(), "output", file) for file in files]

    for file in files:
        exe_path = os.path.join(os.getcwd(), "file_converter.exe")
        print(exe_path + " --deconvert \"" + file + "\"")

        files_to_process += 1
        thread = threading.Thread(target=runInThread, args=(exitDeconvert, exe_path + " --deconvert \"" + file + "\""))
        thread.start()

def exitDeconvert(callback):
    global files_to_process
    files_to_process -= 1
    print(files_to_process, "files remaining")
    if files_to_process == 0:
        print("Finished deconverting")

        for file in os.listdir(os.path.join(os.getcwd(), "output")):
             if ".eimg" in file or ".evid" in file or ".eaud" in file or ".eall" in file or not "." in file:
                    try :
                        os.remove(os.path.join(os.getcwd(), "output", file))
                    except:
                         print("Error removing file: " + file)
        try :
            os.remove(os.path.join(os.getcwd(), "packed.e"))
        except:
            print("Error removing file: packed.e")
                  

        callback()
    