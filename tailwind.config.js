import { fontFamily } from 'tailwindcss/defaultTheme';

/** @type {import('tailwindcss').Config} */
const config = {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            colors: {
                primary: 'var(--primary)',
                onPrimary: 'var(--onPrimary)',
                primaryContainer: 'var(--primaryContainer)',
                onPrimaryContainer: 'var(--onPrimaryContainer)',
                inversePrimary: 'var(--inversePrimary)',
                secondary: 'var(--secondary)',
                onSecondary: 'var(--onSecondary)',
                secondaryContainer: 'var(--secondaryContainer)',
                onSecondaryContainer: 'var(--onSecondaryContainer)',
                tertiary: 'var(--tertiary)',
                onTertiary: 'var(--onTertiary)',
                tertiaryContainer: 'var(--tertiaryContainer)',
                onTertiaryContainer: 'var(--onTertiaryContainer)',
                background: 'var(--background)',
                onBackground: 'var(--onBackground)',
                surface: 'var(--surface)',
                onSurface: 'var(--onSurface)',
                surfaceVariant: 'var(--surfaceVariant)',
                onSurfaceVariant: 'var(--onSurfaceVariant)',
                inverseSurface: 'var(--inverseSurface)',
                inverseOnSurface: 'var(--inverseOnSurface)',
                outline: 'var(--outline)'
            }
        }
    }
};

export default config;
