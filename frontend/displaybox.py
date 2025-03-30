import pygame

BOX_BG_COLOR = "#04080F"
BORDER_COLOR = "#FFFFFF"
TEXT_COLOR = "#FFFFFF"

def DisplayBox(x, y, width, height, data = None, active=False, BG_Color= None,ScreenSize = None,type = None,file_type = None,file_name = None):
    if type == "text":
        return TextDisplayBox(x, y, width, height, data, active, BG_Color= BG_Color, ScreenSize = ScreenSize,file_type = file_type,file_name = file_name)
    elif type == "empty" :
        return EmptyDisplayBox(x, y, width, height, active, BG_Color= BG_Color, ScreenSize = ScreenSize,file_type = file_type,file_name = file_name)
    elif type == "image":
        return ImageDisplayBox(x, y, width, height, active,image_path = data,ScreenSize = ScreenSize,file_type = file_type,file_name = file_name)
    elif type == "not_supported":
        return NotSupportedBox(x, y, width, height, active,file_type = data, file_name = file_name,ScreenSize = ScreenSize)
    
class FileInfo:
    def __init__(self,x,y,width,height,active = False,file_type = None,file_name = None):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.active = active
        self.bg_color = BOX_BG_COLOR
        self.bd_color = BORDER_COLOR
        self.font = pygame.font.Font("frontend/scpfont.ttf", 30)
        self.file_type = file_type
        self.file_name = file_name
    def draw(self, screen):
        if self.active:
            pygame.draw.rect(screen, self.bg_color, (self.x, self.y, self.width, self.height))
            pygame.draw.rect(screen,self.bd_color,(self.x, self.y, self.width, self.height), 2)
            pygame.draw.line(screen, self.bd_color, (self.x + self.width - self.width//8,self.y),(self.x+ self.width - self.width//8,self.height + 8), 2)
            file_name_text = self.font.render(self.file_name, True, TEXT_COLOR)
            file_name_rect = file_name_text.get_rect(center=(self.x + 13*len(self.file_name)//2 + 15 ,self.y + self.height//2))
            screen.blit(file_name_text, file_name_rect)
            file_type_text = self.font.render(self.file_type, True, TEXT_COLOR)
            file_type_rect = file_type_text.get_rect(center=(self.x + self.width - self.width//8 + 13*len(self.file_type)//2 + 15 ,self.y + self.height//2))
            screen.blit(file_type_text, file_type_rect)
        
        

class TextDisplayBox:
    def __init__(self, x, y, width, height, text="", active=False, BG_Color= None,ScreenSize = None, file_type = None,file_name = None):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.bg_color = BOX_BG_COLOR
        self.bd_color = BORDER_COLOR
        self.active = active
        self.scroll_y = 0
        self.scroll_speed = 20
        self.screen_size = ScreenSize
        self.font = pygame.font.Font("frontend/scpfont.ttf", 22)
        self.BackG_COLOR = BG_Color
        self.data = text
        self.file_name = file_name
        self.file_type = file_type
        self.ScrollBarColor = (255, 255, 255)
        self.scroll_porcent = 0
        self.create_text_surface()
        self.file_info = FileInfo(self.x , self.y - self.screen_size[1]//24, self.width, self.screen_size[1]//24, active = active,file_type = self.file_type,file_name = self.file_name)
    def wrap_text(self, text, width):
        words = text.split(" ")
        lines = []
        current_line = ''
        for word in words:
            print("Current line: ", current_line)
            word = word.strip()
            if len(current_line + word)*13 <= width - 20:
                print("Added word: ", word)
                current_line += word + ' '
            elif len(word)*13 > width - 20:
                print("Word too long: ", word)
                too_long_word = word
                for i in range(0, len(too_long_word)):
                    if 13*len(current_line + too_long_word[:i]) >= width - 20:
                        lines.append(current_line + too_long_word[:i-1])
                        current_line = ''
                        too_long_word = too_long_word[i-1:]
                        break
                while 13*len(too_long_word) > width - 20:
                    for i in range(0, len(too_long_word)):
                        if 13*len(too_long_word[:i]) >= width - 20:
                            lines.append(current_line + too_long_word[:i-1])
                            too_long_word = too_long_word[i-1:]
                            break
                current_line = too_long_word + ' '

            else :
                print("Added Current line: ", current_line)
                lines.append(current_line)
                current_line = word + ' '
        lines.append(current_line)
        print(lines)
        return lines
    def create_text_surface(self):
        self.lines = self.wrap_text(self.data, self.width - 20)
        self.text_surface = pygame.Surface((self.width - 25, len(self.lines) * 30))
        self.text_surface.fill(self.bg_color)
        for i, line in enumerate(self.lines):
            if not line :
                continue
            text_surface = self.font.render(line.strip(), True, TEXT_COLOR)
            self.text_surface.blit(text_surface, (0, i * 30))
    def draw(self, screen):
        if self.active:
            pygame.draw.rect(screen, self.bg_color, (self.x + 2, self.y + 2, self.width - 4, self.height - 4))
            screen.blit(self.text_surface, (self.x + 20, self.y + 10 - self.scroll_y))
            pygame.draw.rect(screen, self.BackG_COLOR, (self.x - 1, 0, self.width, self.y + 2))
            pygame.draw.rect(screen, self.BackG_COLOR, (self.x, self.y + self.height - 2, self.width, self.screen_size[1]))
            pygame.draw.rect(screen, self.ScrollBarColor, (self.x + self.width - 10, self.y + self.scroll_porcent, 4, 20))
            pygame.draw.rect(screen, self.bd_color, (self.x, self.y, self.width, self.height), 2)
            self.file_info.draw(screen)

    def handle_event(self, event):
        if event.type == pygame.MOUSEBUTTONDOWN and event.button == 4 and self.scroll_y - self.scroll_speed >= 0:
            self.scroll_y -= self.scroll_speed
        elif event.type == pygame.MOUSEBUTTONDOWN and event.button == 5 and self.scroll_y + self.scroll_speed <= self.text_surface.get_height() - self.height + 20:
            self.scroll_y += self.scroll_speed
        self.scroll_porcent = (self.scroll_y / (self.text_surface.get_height() - self.height)) * (self.height - 30)

class EmptyDisplayBox:
    def __init__(self, x, y, width, height, active=False, BG_Color= None,ScreenSize = None,file_type = None,file_name = None):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.bg_color = BOX_BG_COLOR
        self.bd_color = BORDER_COLOR
        self.active = active
        self.font = pygame.font.Font("frontend/scpfont.ttf", 70)
        self.screen_size = ScreenSize

        self.file_name = file_name
        self.file_type = file_type
        self.file_info = FileInfo(self.x , self.y - self.screen_size[1]//24, self.width, self.screen_size[1]//24, active = active,file_type = "No file selected",file_name ="/")
    def draw(self, screen):
        if self.active:
            self.file_info.draw(screen)
            pygame.draw.rect(screen, self.bg_color, (self.x + 2, self.y + 2, self.width - 4, self.height - 4))
            pygame.draw.rect(screen, self.bd_color, (self.x, self.y, self.width, self.height), 2)
            text = self.font.render("Nothing to display", True, (180, 180, 180))
            text_rect = text.get_rect(center=(self.x + self.width // 2, self.y + self.height // 2))
            screen.blit(text, text_rect)
    def handle_event(self, event):
        pass

class NotSupportedBox:
    def __init__(self, x, y, width, height, active=False,file_type = None, file_name = None,ScreenSize = None):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.bg_color = BOX_BG_COLOR
        self.bd_color = BORDER_COLOR
        self.active = active
        self.font = pygame.font.Font("frontend/scpfont.ttf", 70)
        self.file_type = file_type
        self.file_name = file_name
        self.screen_size = ScreenSize
        self.file_info = FileInfo(self.x , self.y - self.screen_size[1]//24, self.width, self.screen_size[1]//24, active = active,file_type = self.file_type,file_name = self.file_name)
    def draw(self, screen):
        if self.active:
            self.file_info.draw(screen)
            pygame.draw.rect(screen, self.bg_color, (self.x + 2, self.y + 2, self.width - 4, self.height - 4))
            pygame.draw.rect(screen, self.bd_color, (self.x, self.y, self.width, self.height), 2)
            text = self.font.render('FileType "'+self.file_type +'" Not Supported', True, (180, 180, 180))
            text_rect = text.get_rect(center=(self.x + self.width // 2, self.y + self.height // 2))
            screen.blit(text, text_rect)
    def handle_event(self, event):
        pass
class ImageDisplayBox:
    def __init__(self, x, y, width, height, active=False,image_path = None,file_type = None,file_name = None,ScreenSize = None):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.bg_color = BOX_BG_COLOR
        self.bd_color = BORDER_COLOR
        self.active = active
        self.font = pygame.font.Font("frontend/scpfont.ttf", 70)
        self.image = pygame.image.load(image_path)
        self.img_ratio = self.image.get_width() / self.image.get_height()
        self.aspect_img_ration = (self.image.get_width()/(self.width -20), self.image.get_height()/(self.height - 20))
        if max(self.aspect_img_ration) < 1:
            while max(self.aspect_img_ration) < 0.5 :
                self.image = pygame.transform.scale(self.image, (self.image.get_width()*2, self.image.get_height()*2))
                self.aspect_img_ration = (self.image.get_width()/(self.width -20), self.image.get_height()/(self.height - 20))
        else:
            while max(self.aspect_img_ration) > 1:
                self.image = pygame.transform.scale(self.image, (self.image.get_width()/2, self.image.get_height()/2))
                self.aspect_img_ration = (self.image.get_width()/(self.width -20), self.image.get_height()/(self.height - 20))
        self.img_rect = self.image.get_rect(center=(self.x + self.width // 2, self.y + self.height // 2))
        self.file_type = file_type
        self.file_name = file_name
        self.screen_size = ScreenSize
        self.file_info = FileInfo(self.x , self.y - self.screen_size[1]//24, self.width, self.screen_size[1]//24, active = active,file_type = self.file_type,file_name = self.file_name)
        
    def draw(self, screen):
        if self.active:
            self.file_info.draw(screen)
            pygame.draw.rect(screen, self.bg_color, (self.x + 2, self.y + 2, self.width - 4, self.height - 4))
            pygame.draw.rect(screen, self.bd_color, (self.x, self.y, self.width, self.height), 2)
            screen.blit(self.image, self.img_rect)
    def handle_event(self, event):
        pass

