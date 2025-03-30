import pygame

class Button :
    def __init__(self,x,y,width,height,text,bg_color,hover_color,click_color,font_size=30,active=True,command = None): 
        self.text = text
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.padding = 10

        self.bg_color = "#647AA3"
        self.bd_color = "#344A73"
        self.hover_color = "#546A93"
        self.click_color = "#445A83"
        self.font_size = font_size
        self.font = pygame.font.Font(None, font_size)
        self.command = command

        self.hovered = False
        self.clicked = False
        self.active = active
        
    def draw(self, screen):
        color = self.hover_color if self.hovered else self.click_color if self.clicked else self.bg_color

        pygame.draw.rect(screen, self.bd_color if not self.clicked else self.click_color, (self.x, self.y, self.width, self.height))
        rect = (self.x, self.y, self.width, self.height - self.padding)
        pygame.draw.rect(screen, color, rect)

        text_surface = self.font.render(self.text, True, (255, 255, 255))
        text_rect = text_surface.get_rect(center=(self.x + self.width // 2, self.y + self.height // 2))
        screen.blit(text_surface, text_rect)
    
    def update(self, mouse_pos, event):
        mx, my = mouse_pos
        if event == "hover":
            self.hovered = self.x <= mx <= self.x + self.width and self.y <= my <= self.y + self.height
        elif event == "mousedown" and self.hovered:
            self.clicked = True
        elif event == "mouseup" and self.hovered:
            self.clicked = False
            if self.command is not None:
                self.command()
