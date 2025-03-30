import pygame

import subprocess

from tkinter.filedialog import askopenfilename
from tkinter.simpledialog import askstring

from button import Button
from scrollbox import ScrollBox
from displaybox import DisplayBox
from passwordinput import PasswordInput


def draw(screen, buttons, frame):
    for button in buttons:
        if button.active:
            button.draw(screen)
    if Display_Box.active:
        Display_Box.draw(screen)
    if scroll_box.active:
        scroll_box.draw(screen)
    if password_input.active:
        password_input.draw(screen, frame)


def update_buttons(mouse_pos, buttons, event):
    global menu
    old_menu = menu

    hovered = False
    for button in buttons:
        if not button.active:
            continue
        button.update(mouse_pos, event)
        if button.hovered:
            hovered = True
    if hovered:
        pygame.mouse.set_cursor(pygame.SYSTEM_CURSOR_HAND)
    else:
        pygame.mouse.set_cursor(pygame.SYSTEM_CURSOR_ARROW)

    if menu != old_menu:
        pygame.mouse.set_cursor(pygame.SYSTEM_CURSOR_ARROW)
        old_menu = menu


def change_menu(new_menu):
    print("new", new_menu)
    global menu
    menu = new_menu
    for button in buttons:
        button.active = False
        button.hovered = False
    scroll_box.active = False
    Display_Box.active = False
    password_input.active = False
    if menu == "main":
        buttons[0].active = True
        buttons[1].active = True
        buttons[2].active = True
    elif menu == "encrypt":
        Display_Box.active = True
        buttons[3].active = True
        buttons[4].active = True
        buttons[6].active = True
        scroll_box.max_elems = -1
        scroll_box.active = True
        password_input.active = True
    elif menu == "decrypt":
        Display_Box.active = True
        buttons[3].active = True
        buttons[5].active = True
        buttons[6].active = True
        scroll_box.max_elems = 1
        scroll_box.active = True
        password_input.active = True

    update_buttons(pygame.mouse.get_pos(), buttons, "hover")

def add_file():
    file = askopenfilename()
    if not file:
        return
    scroll_box.add_file(file.split("/")[-1], file)
    selected_files.append(file)
    display_file(file)


def display_file(file):
    global Display_Box
    print(file)
    if file.split(".")[-1].lower() == "txt":
        with open(file, "r", encoding="utf-16") as f:
            text = f.read().strip()
            f.close()
        print(text)
        Display_Box = DisplayBox(SCREEN_WIDTH//4, SCREEN_HEIGHT//20, SCREEN_WIDTH//1.5, 18*SCREEN_HEIGHT//20, type = "text",BG_Color=BG_COLOR,
                                 ScreenSize = (SCREEN_WIDTH, SCREEN_HEIGHT), data = text,active = True,file_name=file.split("/")[-1].split(".")[0],file_type=("."+file.split(".")[-1]))
    elif file.split(".")[-1].lower() == "png" or file.split(".")[-1] == "jpg":
        Display_Box = DisplayBox(SCREEN_WIDTH//4, SCREEN_HEIGHT//20, SCREEN_WIDTH//1.5, 18*SCREEN_HEIGHT//20, type = "image",BG_Color=BG_COLOR,
                                 ScreenSize = (SCREEN_WIDTH, SCREEN_HEIGHT), data = file,active = True,file_name=file.split("/")[-1].split(".")[0],file_type=("."+file.split(".")[-1]))
    else :
        Display_Box = DisplayBox(SCREEN_WIDTH//4, SCREEN_HEIGHT//20, SCREEN_WIDTH//1.5, 18*SCREEN_HEIGHT//20, type = "not_supported",BG_Color=BG_COLOR,
                                 ScreenSize = (SCREEN_WIDTH, SCREEN_HEIGHT), data = ("."+file.split(".")[-1]),active = True,file_name=file.split("/")[-1].split(".")[0],file_type=("."+file.split(".")[-1]))

def encrypt_files():
    exe_path = r"C:\path\to\program.exe"
    args = ["arg1"]

    subprocess.run([exe_path] + args)


def decrypt_file():
    if len(scroll_box.files) == 0:
        return
    password = askstring("", "Password")

    exe_path = r"C:\path\to\program.exe"
    args = ["arg1"]

    subprocess.run([exe_path] + args)


def quit():
    pygame.quit()
    exit()


if __name__ == "__main__":
    pygame.display.init()
    pygame.font.init()

    screen = pygame.display.set_mode(flags=pygame.FULLSCREEN)
    pygame.display.set_caption("vaulternal")
    SCREEN_WIDTH, SCREEN_HEIGHT = screen.get_size()
    BG_COLOR = "#24282F"
    
    pygame.scrap.init()
    pygame.scrap.set_mode(pygame.SCRAP_CLIPBOARD)

    clock = pygame.time.Clock()

    menu = "main" # "main", "decrypt", "encrypt"
    buttons = []
    selected_files  = []

    Encrypt_Main_Button = Button(SCREEN_WIDTH//2-SCREEN_WIDTH//16, SCREEN_HEIGHT//2-SCREEN_HEIGHT//10 - SCREEN_HEIGHT//10, SCREEN_WIDTH//8, SCREEN_HEIGHT//10, 
                                      "Encrypt", (0, 255, 0), (0, 200, 0), (0, 150, 0),command = lambda: change_menu("encrypt"))
    buttons.append(Encrypt_Main_Button) #0
    Decrypt_Main_Button = Button(SCREEN_WIDTH//2-SCREEN_WIDTH//16, SCREEN_HEIGHT//2-SCREEN_HEIGHT//10 + SCREEN_HEIGHT//10, SCREEN_WIDTH//8, SCREEN_HEIGHT//10, 
                                      "Decrypt", (0, 255, 0), (0, 200, 0), (0, 150, 0,),command = lambda: change_menu("decrypt"))
    buttons.append(Decrypt_Main_Button) #1

    Close_Button = Button(SCREEN_WIDTH-SCREEN_WIDTH//13,SCREEN_HEIGHT//50 , SCREEN_WIDTH//15, SCREEN_HEIGHT//20,
                                      "Close", (255, 0, 0), (200, 0, 0), (150, 0, 0),command = lambda: quit())
    buttons.append(Close_Button) #2

    Display_Box = DisplayBox(SCREEN_WIDTH//4, SCREEN_HEIGHT//20, SCREEN_WIDTH//1.5, 18*SCREEN_HEIGHT//20, 
                             type = "empty",BG_Color=BG_COLOR,ScreenSize = (SCREEN_WIDTH, SCREEN_HEIGHT), data = None)

    scroll_box = ScrollBox(SCREEN_WIDTH//22, SCREEN_HEIGHT//2 - 2.5*SCREEN_HEIGHT//10 , SCREEN_WIDTH//6,SCREEN_HEIGHT//2 - SCREEN_HEIGHT//10)

    password_input = PasswordInput(SCREEN_WIDTH//22, SCREEN_HEIGHT//2 + SCREEN_HEIGHT//5, SCREEN_WIDTH//6, SCREEN_HEIGHT//20)

    password_label = pygame.font.Font(None, 30).render("Password :", True, (255, 255, 255))
    password_label_rect = password_label.get_rect(center=(SCREEN_WIDTH//22 + SCREEN_WIDTH//12, SCREEN_HEIGHT//2 + SCREEN_HEIGHT//5 - SCREEN_HEIGHT//40))

    open_file_button = Button(SCREEN_WIDTH//22, SCREEN_HEIGHT//8 ,SCREEN_WIDTH//6, SCREEN_HEIGHT//10,
                                      "Open File", (0, 255, 0), (0, 200, 0), (0, 150, 0),command = lambda: add_file(), active = False)
    buttons.append(open_file_button) #3
    
    encrypt_button = Button(SCREEN_WIDTH//22, SCREEN_HEIGHT - SCREEN_HEIGHT//10 - SCREEN_HEIGHT//8 ,SCREEN_WIDTH//6, SCREEN_HEIGHT//10,
                                      "Encrypt Files", (0, 255, 0), (0, 200, 0), (0, 150, 0),command = lambda: encrypt_files(), active = False)
    buttons.append(encrypt_button) #4

    decrypt_button = Button(SCREEN_WIDTH//22, SCREEN_HEIGHT - SCREEN_HEIGHT//10 - SCREEN_HEIGHT//8 ,SCREEN_WIDTH//6, SCREEN_HEIGHT//10,
                                      "Decrypt File", (0, 255, 0), (0, 200, 0), (0, 150, 0),command = lambda: decrypt_file(), active = False)
    buttons.append(decrypt_button) #5

    Back_Button = Button(SCREEN_WIDTH-SCREEN_WIDTH//13,SCREEN_HEIGHT//50 , SCREEN_WIDTH//15, SCREEN_HEIGHT//20,
                                      "Back", (255, 0, 0), (200, 0, 0), (150, 0, 0),command = lambda: change_menu("main"), active=False)
    buttons.append(Back_Button) #6
    
    title = pygame.font.Font(None, 100).render("VAULTERNAL", True, (255, 255, 255))
    title_rect = title.get_rect(center=(SCREEN_WIDTH//2, SCREEN_HEIGHT//10))

    subtitle = pygame.font.Font("frontend/scpfont.ttf", 30).render("Please select an option", True, (255, 255, 255))
    subtitle_rect = subtitle.get_rect(center=(SCREEN_WIDTH//2, SCREEN_HEIGHT//6))
    frame = 0
    
    while True:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                quit()

            Display_Box.handle_event(event)

            if event.type == pygame.MOUSEMOTION:
                mouse_pos = event.pos
                update_buttons(mouse_pos, buttons, "hover")

            if event.type == pygame.MOUSEBUTTONDOWN and event.button == 1:
                mouse_pos = event.pos
                update_buttons(mouse_pos, buttons, "mousedown")

            if event.type == pygame.MOUSEBUTTONUP:
                if event.button == 1:
                    mouse_pos = event.pos
                    update_buttons(mouse_pos, buttons, "mouseup")
                    result = scroll_box.click(mouse_pos)
                    if result == "deleted":
                        Display_Box = DisplayBox(SCREEN_WIDTH//4, SCREEN_HEIGHT//20, SCREEN_WIDTH//1.5, 18*SCREEN_HEIGHT//20, 
                              type = "empty",BG_Color=BG_COLOR,ScreenSize = (SCREEN_WIDTH, SCREEN_HEIGHT), data = None,active = True)
                    elif result is not None:
                        file = result
                        display_file(file.path)
                    password_input.click(mouse_pos)
                if event.button == 4:
                    scroll_box.scroll(-1)
                if event.button == 5:
                    scroll_box.scroll(1)
            
            if event.type == pygame.KEYDOWN:
                if password_input.active and password_input.selected:
                    if event.key in (pygame.K_ESCAPE, pygame.K_RETURN):
                        password_input.deselect()
                    elif event.key == pygame.K_BACKSPACE:
                        password_input.remove_char()
                    elif event.key == pygame.K_v and event.mod == pygame.KMOD_CTRL:
                        password_input.add_text(pygame.scrap.get("text/plain;charset=utf-8").decode())
                    else:
                        password_input.add_text(event.unicode)

        screen.fill(BG_COLOR)
        if menu == "main":
            screen.blit(title, title_rect)
            screen.blit(subtitle, subtitle_rect)
        else :
            screen.blit(password_label, password_label_rect)
        draw(screen, buttons, frame)
        pygame.display.flip()
        clock.tick(30)
        frame += 1