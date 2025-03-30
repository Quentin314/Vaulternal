import pygame

BG_COLOR = "#04080F"
SELECTED_COLOR = "#14181F"
TEXT_COLOR = "#FFFFFF"

class PasswordInput:
    def __init__(self, x, y, width, height):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.padding = 5

        self.active = False
        self.selected = False

        self.visible = True

        self.password = ""
        self.font = pygame.font.Font(None, self.height - self.padding)

    def draw(self, screen, frame):
        if not self.active:
            return
        pygame.draw.rect(screen, BG_COLOR if not self.selected else SELECTED_COLOR, (self.x, self.y, self.width, self.height))
        pygame.draw.rect(screen, TEXT_COLOR, (self.x, self.y, self.width, self.height), 2)

        text = self.password if self.visible else "•" * len(self.password)

        i = len(text)
        while self.font.size(text[:i] + "_")[0] >= self.width - self.padding * 2:
            i -= 1

        if self.selected and frame//15 % 2:
            text += "_"
        render_text = self.font.render(text, True, TEXT_COLOR)
        screen.blit(render_text,  (self.x + self.padding + 2, self.y + self.padding))
    
    def add_text(self, text):
        self.password += text

    def remove_char(self):
        self.password = self.password[:-1]
    
    def deselect(self):
        self.selected = False

    def click(self, mouse_pos):
        mx, my = mouse_pos
        if not self.active or not (self.x < mx < self.x + self.width and self.y < my < self.y + self.height):
            return
        self.selected = True